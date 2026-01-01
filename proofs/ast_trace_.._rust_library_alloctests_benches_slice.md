# AST Trace: ../rust/library/alloctests/benches/slice.rs

Generated 36 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use std::ptr;

use rand::Rng;
use rand::distr::{Alphanumeric, SampleString, StandardUniform};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use test::{Bencher, black_box};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=iterator | COMPLEXITY=9 | LINES=18

```rust
#[bench]
fn iterator(b: &mut Bencher) {
    // peculiar numbers to stop LLVM from optimising the summation
    // out.
    let v: Vec<_> = (0..100).map(|i| i ^ (i << 1) ^ (i >> 1)).collect();

    b.iter(|| {
        let mut sum = 0;
        for x in &v {
            sum += *x;
        }
        // sum == 11806, to stop dead code elimination.
        if sum == 0 {
            panic!()
        }
    })
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=mut_iterator | COMPLEXITY=6 | LINES=13

```rust
#[bench]
fn mut_iterator(b: &mut Bencher) {
    let mut v = vec![0; 100];

    b.iter(|| {
        let mut i = 0;
        for x in &mut v {
            *x = i;
            i += 1;
        }
    })
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=concat | COMPLEXITY=3 | LINES=8

```rust
#[bench]
fn concat(b: &mut Bencher) {
    let xss: Vec<Vec<i32>> = (0..100).map(|i| (0..i).collect()).collect();
    b.iter(|| {
        xss.concat();
    });
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=join | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn join(b: &mut Bencher) {
    let xss: Vec<Vec<i32>> = (0..100).map(|i| (0..i).collect()).collect();
    b.iter(|| xss.join(&0));
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=push | COMPLEXITY=3 | LINES=9

```rust
#[bench]
fn push(b: &mut Bencher) {
    let mut vec = Vec::<i32>::new();
    b.iter(|| {
        vec.push(0);
        black_box(&vec);
    });
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=starts_with_same_vector | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn starts_with_same_vector(b: &mut Bencher) {
    let vec: Vec<_> = (0..100).collect();
    b.iter(|| vec.starts_with(&vec))
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=starts_with_single_element | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn starts_with_single_element(b: &mut Bencher) {
    let vec: Vec<_> = vec![0];
    b.iter(|| vec.starts_with(&vec))
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=starts_with_diff_one_element_at_end | COMPLEXITY=2 | LINES=8

```rust
#[bench]
fn starts_with_diff_one_element_at_end(b: &mut Bencher) {
    let vec: Vec<_> = (0..100).collect();
    let mut match_vec: Vec<_> = (0..99).collect();
    match_vec.push(0);
    b.iter(|| vec.starts_with(&match_vec))
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=ends_with_same_vector | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn ends_with_same_vector(b: &mut Bencher) {
    let vec: Vec<_> = (0..100).collect();
    b.iter(|| vec.ends_with(&vec))
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=ends_with_single_element | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn ends_with_single_element(b: &mut Bencher) {
    let vec: Vec<_> = vec![0];
    b.iter(|| vec.ends_with(&vec))
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=ends_with_diff_one_element_at_beginning | COMPLEXITY=2 | LINES=8

```rust
#[bench]
fn ends_with_diff_one_element_at_beginning(b: &mut Bencher) {
    let vec: Vec<_> = (0..100).collect();
    let mut match_vec: Vec<_> = (0..100).collect();
    match_vec[0] = 200;
    b.iter(|| vec.starts_with(&match_vec))
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=contains_last_element | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn contains_last_element(b: &mut Bencher) {
    let vec: Vec<_> = (0..100).collect();
    b.iter(|| vec.contains(&99))
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=zero_1kb_from_elem | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn zero_1kb_from_elem(b: &mut Bencher) {
    b.iter(|| vec![0u8; 1024]);
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=zero_1kb_set_memory | COMPLEXITY=8 | LINES=13

```rust
#[bench]
fn zero_1kb_set_memory(b: &mut Bencher) {
    b.iter(|| {
        let mut v = Vec::<u8>::with_capacity(1024);
        unsafe {
            let vp = v.as_mut_ptr();
            ptr::write_bytes(vp, 0, 1024);
            v.set_len(1024);
        }
        v
    });
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=zero_1kb_loop_set | COMPLEXITY=11 | LINES=13

```rust
#[bench]
fn zero_1kb_loop_set(b: &mut Bencher) {
    b.iter(|| {
        let mut v = Vec::<u8>::with_capacity(1024);
        unsafe {
            v.set_len(1024);
        }
        for i in 0..1024 {
            v[i] = 0;
        }
    });
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=zero_1kb_mut_iter | COMPLEXITY=11 | LINES=14

```rust
#[bench]
fn zero_1kb_mut_iter(b: &mut Bencher) {
    b.iter(|| {
        let mut v = Vec::<u8>::with_capacity(1024);
        unsafe {
            v.set_len(1024);
        }
        for x in &mut v {
            *x = 0;
        }
        v
    });
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=random_inserts | COMPLEXITY=6 | LINES=12

```rust
#[bench]
fn random_inserts(b: &mut Bencher) {
    let mut rng = crate::bench_rng();
    b.iter(|| {
        let mut v = vec![(0, 0); 30];
        for _ in 0..100 {
            let l = v.len();
            v.insert(rng.random::<u32>() as usize % (l + 1), (1, 1));
        }
    })
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=random_removes | COMPLEXITY=6 | LINES=12

```rust
#[bench]
fn random_removes(b: &mut Bencher) {
    let mut rng = crate::bench_rng();
    b.iter(|| {
        let mut v = vec![(0, 0); 130];
        for _ in 0..100 {
            let l = v.len();
            v.remove(rng.random::<u32>() as usize % l);
        }
    })
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=gen_ascending | COMPLEXITY=2 | LINES=4

```rust
fn gen_ascending(len: usize) -> Vec<u64> {
    (0..len as u64).collect()
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=gen_descending | COMPLEXITY=2 | LINES=4

```rust
fn gen_descending(len: usize) -> Vec<u64> {
    (0..len as u64).rev().collect()
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=gen_random | COMPLEXITY=2 | LINES=5

```rust
fn gen_random(len: usize) -> Vec<u64> {
    let mut rng = crate::bench_rng();
    (&mut rng).sample_iter(&StandardUniform).take(len).collect()
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=gen_random_bytes | COMPLEXITY=2 | LINES=5

```rust
fn gen_random_bytes(len: usize) -> Vec<u8> {
    let mut rng = crate::bench_rng();
    (&mut rng).sample_iter(&StandardUniform).take(len).collect()
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=gen_mostly_ascending | COMPLEXITY=5 | LINES=11

```rust
fn gen_mostly_ascending(len: usize) -> Vec<u64> {
    let mut rng = crate::bench_rng();
    let mut v = gen_ascending(len);
    for _ in (0usize..).take_while(|x| x * x <= len) {
        let x = rng.random::<u32>() as usize % len;
        let y = rng.random::<u32>() as usize % len;
        v.swap(x, y);
    }
    v
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=gen_mostly_descending | COMPLEXITY=5 | LINES=11

```rust
fn gen_mostly_descending(len: usize) -> Vec<u64> {
    let mut rng = crate::bench_rng();
    let mut v = gen_descending(len);
    for _ in (0usize..).take_while(|x| x * x <= len) {
        let x = rng.random::<u32>() as usize % len;
        let y = rng.random::<u32>() as usize % len;
        v.swap(x, y);
    }
    v
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=gen_strings | COMPLEXITY=5 | LINES=10

```rust
fn gen_strings(len: usize) -> Vec<String> {
    let mut rng = crate::bench_rng();
    let mut v = vec![];
    for _ in 0..len {
        let n = rng.random::<u32>() % 20 + 1;
        v.push(Alphanumeric.sample_string(&mut rng, n as usize));
    }
    v
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=gen_big_random | COMPLEXITY=2 | LINES=5

```rust
fn gen_big_random(len: usize) -> Vec<[u64; 16]> {
    let mut rng = crate::bench_rng();
    (&mut rng).sample_iter(&StandardUniform).map(|x| [x; 16]).take(len).collect()
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=9 | LINES=11

```rust
macro_rules! sort {
    ($f:ident, $name:ident, $gen:expr, $len:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let v = $gen($len);
            b.iter(|| v.clone().$f());
            b.bytes = $len * size_of_val(&$gen(1)[0]) as u64;
        }
    };
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=9 | LINES=12

```rust
macro_rules! sort_strings {
    ($f:ident, $name:ident, $gen:expr, $len:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let v = $gen($len);
            let v = v.iter().map(|s| &**s).collect::<Vec<&str>>();
            b.iter(|| v.clone().$f());
            b.bytes = $len * size_of::<&str>() as u64;
        }
    };
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=15 | LINES=22

```rust
macro_rules! sort_expensive {
    ($f:ident, $name:ident, $gen:expr, $len:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let v = $gen($len);
            b.iter(|| {
                let mut v = v.clone();
                let mut count = 0;
                v.$f(|a: &u64, b: &u64| {
                    count += 1;
                    if count % 1_000_000_000 == 0 {
                        panic!("should not happen");
                    }
                    (*a as f64).cos().partial_cmp(&(*b as f64).cos()).unwrap()
                });
                black_box(count);
            });
            b.bytes = $len * size_of_val(&$gen(1)[0]) as u64;
        }
    };
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=9 | LINES=11

```rust
macro_rules! sort_lexicographic {
    ($f:ident, $name:ident, $gen:expr, $len:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let v = $gen($len);
            b.iter(|| v.clone().$f(|x| x.to_string()));
            b.bytes = $len * size_of_val(&$gen(1)[0]) as u64;
        }
    };
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=13 | LINES=45

```rust
sort!(sort, sort_small_ascending, gen_ascending, 10);
sort!(sort, sort_small_descending, gen_descending, 10);
sort!(sort, sort_small_random, gen_random, 10);
sort!(sort, sort_small_big, gen_big_random, 10);
sort!(sort, sort_medium_random, gen_random, 100);
sort!(sort, sort_large_ascending, gen_ascending, 10000);
sort!(sort, sort_large_descending, gen_descending, 10000);
sort!(sort, sort_large_mostly_ascending, gen_mostly_ascending, 10000);
sort!(sort, sort_large_mostly_descending, gen_mostly_descending, 10000);
sort!(sort, sort_large_random, gen_random, 10000);
sort!(sort, sort_large_big, gen_big_random, 10000);
sort_strings!(sort, sort_large_strings, gen_strings, 10000);
sort_expensive!(sort_by, sort_large_expensive, gen_random, 10000);

sort!(sort_unstable, sort_unstable_small_ascending, gen_ascending, 10);
sort!(sort_unstable, sort_unstable_small_descending, gen_descending, 10);
sort!(sort_unstable, sort_unstable_small_random, gen_random, 10);
sort!(sort_unstable, sort_unstable_small_big, gen_big_random, 10);
sort!(sort_unstable, sort_unstable_medium_random, gen_random, 100);
sort!(sort_unstable, sort_unstable_large_ascending, gen_ascending, 10000);
sort!(sort_unstable, sort_unstable_large_descending, gen_descending, 10000);
sort!(sort_unstable, sort_unstable_large_mostly_ascending, gen_mostly_ascending, 10000);
sort!(sort_unstable, sort_unstable_large_mostly_descending, gen_mostly_descending, 10000);
sort!(sort_unstable, sort_unstable_large_random, gen_random, 10000);
sort!(sort_unstable, sort_unstable_large_big, gen_big_random, 10000);
sort_strings!(sort_unstable, sort_unstable_large_strings, gen_strings, 10000);
sort_expensive!(sort_unstable_by, sort_unstable_large_expensive, gen_random, 10000);

sort_lexicographic!(sort_by_key, sort_by_key_lexicographic, gen_random, 10000);
sort_lexicographic!(sort_unstable_by_key, sort_unstable_by_key_lexicographic, gen_random, 10000);
sort_lexicographic!(sort_by_cached_key, sort_by_cached_key_lexicographic, gen_random, 10000);

macro_rules! reverse {
    ($name:ident, $ty:ty, $f:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            // odd length and offset by 1 to be as unaligned as possible
            let n = 0xFFFFF;
            let mut v: Vec<_> = (0..1 + (n / size_of::<$ty>() as u64)).map($f).collect();
            b.iter(|| black_box(&mut v[1..]).reverse());
            b.bytes = n;
        }
    };
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=F64x4([f64; | COMPLEXITY=2 | LINES=13

```rust
reverse!(reverse_u8, u8, |x| x as u8);
reverse!(reverse_u16, u16, |x| x as u16);
reverse!(reverse_u8x3, [u8; 3], |x| [x as u8, (x >> 8) as u8, (x >> 16) as u8]);
reverse!(reverse_u32, u32, |x| x as u32);
reverse!(reverse_u64, u64, |x| x as u64);
reverse!(reverse_u128, u128, |x| x as u128);
#[repr(simd)]
struct F64x4([f64; 4]);
reverse!(reverse_simd_f64x4, F64x4, |x| {
    let x = x as f64;
    F64x4([x, x, x, x])
});
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=$name | COMPLEXITY=9 | LINES=12

```rust
macro_rules! rotate {
    ($name:ident, $gen:expr, $len:expr, $mid:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let size = size_of_val(&$gen(1)[0]);
            let mut v = $gen($len * 8 / size);
            b.iter(|| black_box(&mut v).rotate_left(($mid * 8 + size - 1) / size));
            b.bytes = (v.len() * size) as u64;
        }
    };
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=35

```rust
rotate!(rotate_tiny_by1, gen_random, 16, 1);
rotate!(rotate_tiny_half, gen_random, 16, 16 / 2);
rotate!(rotate_tiny_half_plus_one, gen_random, 16, 16 / 2 + 1);

rotate!(rotate_medium_by1, gen_random, 9158, 1);
rotate!(rotate_medium_by727_u64, gen_random, 9158, 727);
rotate!(rotate_medium_by727_bytes, gen_random_bytes, 9158, 727);
rotate!(rotate_medium_by727_strings, gen_strings, 9158, 727);
rotate!(rotate_medium_half, gen_random, 9158, 9158 / 2);
rotate!(rotate_medium_half_plus_one, gen_random, 9158, 9158 / 2 + 1);

// Intended to use more RAM than the machine has cache
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by1, gen_random, 5 * 1024 * 1024, 1);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by9199_u64, gen_random, 5 * 1024 * 1024, 9199);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by9199_bytes, gen_random_bytes, 5 * 1024 * 1024, 9199);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by9199_strings, gen_strings, 5 * 1024 * 1024, 9199);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by9199_big, gen_big_random, 5 * 1024 * 1024, 9199);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by1234577_u64, gen_random, 5 * 1024 * 1024, 1234577);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by1234577_bytes, gen_random_bytes, 5 * 1024 * 1024, 1234577);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by1234577_strings, gen_strings, 5 * 1024 * 1024, 1234577);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_by1234577_big, gen_big_random, 5 * 1024 * 1024, 1234577);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_half, gen_random, 5 * 1024 * 1024, 5 * 1024 * 1024 / 2);
#[cfg(not(target_os = "emscripten"))] // hits an OOM
rotate!(rotate_huge_half_plus_one, gen_random, 5 * 1024 * 1024, 5 * 1024 * 1024 / 2 + 1);
```

---
*Generated by AST tracing system*
