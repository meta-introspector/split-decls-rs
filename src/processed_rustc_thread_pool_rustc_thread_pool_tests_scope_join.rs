// SRC: ../rust/compiler/rustc_thread_pool/tests/scope_join.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=pseudo_join | COMPLEXITY=3 | LINES=13 */
#[allow(unused_crate_dependencies)]

/// Test that one can emulate join with `scope`:
fn pseudo_join<F, G>(f: F, g: G)
where
    F: FnOnce() + Send,
    G: FnOnce() + Send,
{
    crate::rustc_thread_pool::scope(|s| {
        s.spawn(|_| g());
        f();
    });
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=quick_sort | COMPLEXITY=5 | LINES=10 */

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    pseudo_join(|| quick_sort(lo), || quick_sort(hi));
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
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=is_sorted | COMPLEXITY=2 | LINES=4 */

fn is_sorted<T: Send + Ord>(v: &[T]) -> bool {
    (1..v.len()).all(|i| v[i - 1] <= v[i])
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=scope_join | COMPLEXITY=2 | LINES=7 */

#[test]
fn scope_join() {
    let mut v: Vec<i32> = (0..256).rev().collect();
    quick_sort(&mut v);
    assert!(is_sorted(&v));
}