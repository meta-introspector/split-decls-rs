// SRC: ../rust/compiler/rustc_thread_pool/src/join/tests.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */
// Tests for the join code.

use rand::distr::StandardUniform;
use rand::{Rng, SeedableRng};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=quick_sort | COMPLEXITY=5 | LINES=14 */
use rand_xorshift::XorShiftRng;

use super::*;
use crate::ThreadPoolBuilder;

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    join(|| quick_sort(lo), || quick_sort(hi));
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=partition | COMPLEXITY=8 | LINES=13 */

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=seeded_rng | COMPLEXITY=2 | LINES=6 */

fn seeded_rng() -> XorShiftRng {
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=sort | COMPLEXITY=2 | LINES=10 */

#[test]
fn sort() {
    let rng = seeded_rng();
    let mut data: Vec<u32> = rng.sample_iter(&StandardUniform).take(6 * 1024).collect();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    quick_sort(&mut data);
    assert_eq!(data, sorted_data);
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=sort_in_pool | COMPLEXITY=2 | LINES=13 */

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn sort_in_pool() {
    let rng = seeded_rng();
    let mut data: Vec<u32> = rng.sample_iter(&StandardUniform).take(12 * 1024).collect();

    let pool = ThreadPoolBuilder::new().build().unwrap();
    let mut sorted_data = data.clone();
    sorted_data.sort();
    pool.install(|| quick_sort(&mut data));
    assert_eq!(data, sorted_data);
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=panic_propagate_a | COMPLEXITY=2 | LINES=6 */

#[test]
#[should_panic(expected = "Hello, world!")]
fn panic_propagate_a() {
    join(|| panic!("Hello, world!"), || ());
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=panic_propagate_b | COMPLEXITY=2 | LINES=6 */

#[test]
#[should_panic(expected = "Hello, world!")]
fn panic_propagate_b() {
    join(|| (), || panic!("Hello, world!"));
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=panic_propagate_both | COMPLEXITY=2 | LINES=6 */

#[test]
#[should_panic(expected = "Hello, world!")]
fn panic_propagate_both() {
    join(|| panic!("Hello, world!"), || panic!("Goodbye, world!"));
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=panic_b_still_executes | COMPLEXITY=6 | LINES=10 */

#[test]
#[cfg_attr(not(panic = "unwind"), ignore)]
fn panic_b_still_executes() {
    let mut x = false;
    match unwind::halt_unwinding(|| join(|| panic!("Hello, world!"), || x = true)) {
        Ok(_) => panic!("failed to propagate panic from closure A,"),
        Err(_) => assert!(x, "closure b failed to execute"),
    }
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=join_context_both | COMPLEXITY=2 | LINES=9 */

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn join_context_both() {
    // If we're not in a pool, both should be marked stolen as they're injected.
    let (a_migrated, b_migrated) = join_context(|a| a.migrated(), |b| b.migrated());
    assert!(a_migrated);
    assert!(b_migrated);
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=join_context_neither | COMPLEXITY=2 | LINES=13 */

// FIXME: We should fix or remove this ignored test.
#[test]
#[ignore]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn join_context_neither() {
    // If we're already in a 1-thread pool, neither job should be stolen.
    let pool = ThreadPoolBuilder::new().num_threads(1).build().unwrap();
    let (a_migrated, b_migrated) =
        pool.install(|| join_context(|a| a.migrated(), |b| b.migrated()));
    assert!(!a_migrated);
    assert!(!b_migrated);
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=join_context_second | COMPLEXITY=6 | LINES=24 */

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn join_context_second() {
    use std::sync::Barrier;

    // If we're already in a 2-thread pool, the second job should be stolen.
    let barrier = Barrier::new(2);
    let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();
    let (a_migrated, b_migrated) = pool.install(|| {
        join_context(
            |a| {
                barrier.wait();
                a.migrated()
            },
            |b| {
                barrier.wait();
                b.migrated()
            },
        )
    });
    assert!(!a_migrated);
    assert!(b_migrated);
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=join_counter_overflow | COMPLEXITY=6 | LINES=19 */

#[test]
#[cfg_attr(any(target_os = "emscripten", target_family = "wasm"), ignore)]
fn join_counter_overflow() {
    const MAX: u32 = 500_000;

    let mut i = 0;
    let mut j = 0;
    let pool = ThreadPoolBuilder::new().num_threads(2).build().unwrap();

    // Hammer on join a bunch of times -- used to hit overflow debug-assertions
    // in JEC on 32-bit targets: https://github.com/rayon-rs/rayon/issues/797
    for _ in 0..MAX {
        pool.join(|| i += 1, || j += 1);
    }

    assert_eq!(i, MAX);
    assert_eq!(j, MAX);
}