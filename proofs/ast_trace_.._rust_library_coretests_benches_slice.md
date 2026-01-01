# AST Trace: ../rust/library/coretests/benches/slice.rs

Generated 19 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use core::ptr::NonNull;

use test::{Bencher, black_box};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
enum Cache {
    L1,
    L2,
    L3,
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=size | COMPLEXITY=7 | LINES=10

```rust
impl Cache {
    fn size(&self) -> usize {
        match self {
            Cache::L1 => 1000,      // 8kb
            Cache::L2 => 10_000,    // 80kb
            Cache::L3 => 1_000_000, // 8Mb
        }
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=binary_search | COMPLEXITY=4 | LINES=16

```rust
fn binary_search<F>(b: &mut Bencher, cache: Cache, mapper: F)
where
    F: Fn(usize) -> usize,
{
    let size = cache.size();
    let v = (0..size).map(&mapper).collect::<Vec<_>>();
    let mut r = 0usize;
    b.iter(move || {
        // LCG constants from https://en.wikipedia.org/wiki/Numerical_Recipes.
        r = r.wrapping_mul(1664525).wrapping_add(1013904223);
        // Lookup the whole range to get 50% hits and 50% misses.
        let i = mapper(r % size);
        black_box(v.binary_search(&i).is_ok());
    });
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=binary_search_worst_case | COMPLEXITY=3 | LINES=11

```rust
fn binary_search_worst_case(b: &mut Bencher, cache: Cache) {
    let size = cache.size();

    let mut v = vec![0; size];
    let i = 1;
    v[size - 1] = i;
    b.iter(move || {
        black_box(v.binary_search(&i).is_ok());
    });
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=binary_search_l1 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l1(b: &mut Bencher) {
    binary_search(b, Cache::L1, |i| i * 2);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=binary_search_l2 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l2(b: &mut Bencher) {
    binary_search(b, Cache::L2, |i| i * 2);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=binary_search_l3 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l3(b: &mut Bencher) {
    binary_search(b, Cache::L3, |i| i * 2);
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=binary_search_l1_with_dups | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l1_with_dups(b: &mut Bencher) {
    binary_search(b, Cache::L1, |i| i / 16 * 16);
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=binary_search_l2_with_dups | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l2_with_dups(b: &mut Bencher) {
    binary_search(b, Cache::L2, |i| i / 16 * 16);
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=binary_search_l3_with_dups | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l3_with_dups(b: &mut Bencher) {
    binary_search(b, Cache::L3, |i| i / 16 * 16);
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=binary_search_l1_worst_case | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l1_worst_case(b: &mut Bencher) {
    binary_search_worst_case(b, Cache::L1);
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=binary_search_l2_worst_case | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l2_worst_case(b: &mut Bencher) {
    binary_search_worst_case(b, Cache::L2);
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=binary_search_l3_worst_case | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn binary_search_l3_worst_case(b: &mut Bencher) {
    binary_search_worst_case(b, Cache::L3);
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=Rgb(#[allow(dead_code)] | COMPLEXITY=3 | LINES=9

```rust
#[derive(Clone)]
struct Rgb(#[allow(dead_code)] u8, #[allow(dead_code)] u8, #[allow(dead_code)] u8);

impl Rgb {
    fn new(i: usize) -> Self {
        Rgb(i as u8, (i as u8).wrapping_add(7), (i as u8).wrapping_add(42))
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=$fn | COMPLEXITY=13 | LINES=15

```rust
macro_rules! rotate {
    ($fn:ident, $n:expr, $mapper:expr) => {
        #[bench]
        fn $fn(b: &mut Bencher) {
            let mut x = (0usize..$n).map(&$mapper).collect::<Vec<_>>();
            b.iter(|| {
                for s in 0..x.len() {
                    x[..].rotate_right(s);
                }
                black_box(x[0].clone())
            })
        }
    };
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=$fn | COMPLEXITY=14 | LINES=26

```rust
rotate!(rotate_u8, 32, |i| i as u8);
rotate!(rotate_rgb, 32, Rgb::new);
rotate!(rotate_usize, 32, |i| i);
rotate!(rotate_16_usize_4, 16, |i| [i; 4]);
rotate!(rotate_16_usize_5, 16, |i| [i; 5]);
rotate!(rotate_64_usize_4, 64, |i| [i; 4]);
rotate!(rotate_64_usize_5, 64, |i| [i; 5]);

macro_rules! swap_with_slice {
    ($fn:ident, $n:expr, $mapper:expr) => {
        #[bench]
        fn $fn(b: &mut Bencher) {
            let mut x = (0usize..$n).map(&$mapper).collect::<Vec<_>>();
            let mut y = ($n..($n * 2)).map(&$mapper).collect::<Vec<_>>();
            let mut skip = 0;
            b.iter(|| {
                for _ in 0..32 {
                    x[skip..].swap_with_slice(&mut y[..($n - skip)]);
                    skip = black_box(skip + 1) % 8;
                }
                black_box((x[$n / 3].clone(), y[$n * 2 / 3].clone()))
            })
        }
    };
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=fill_byte_sized | COMPLEXITY=4 | LINES=24

```rust
swap_with_slice!(swap_with_slice_u8_30, 30, |i| i as u8);
swap_with_slice!(swap_with_slice_u8_3000, 3000, |i| i as u8);
swap_with_slice!(swap_with_slice_rgb_30, 30, Rgb::new);
swap_with_slice!(swap_with_slice_rgb_3000, 3000, Rgb::new);
swap_with_slice!(swap_with_slice_usize_30, 30, |i| i);
swap_with_slice!(swap_with_slice_usize_3000, 3000, |i| i);
swap_with_slice!(swap_with_slice_4x_usize_30, 30, |i| [i; 4]);
swap_with_slice!(swap_with_slice_4x_usize_3000, 3000, |i| [i; 4]);
swap_with_slice!(swap_with_slice_5x_usize_30, 30, |i| [i; 5]);
swap_with_slice!(swap_with_slice_5x_usize_3000, 3000, |i| [i; 5]);

#[bench]
fn fill_byte_sized(b: &mut Bencher) {
    #[derive(Copy, Clone)]
    struct NewType(#[allow(dead_code)] u8);

    let mut ary = [NewType(0); 1024];

    b.iter(|| {
        let slice = &mut ary[..];
        black_box(slice.fill(black_box(NewType(42))));
    });
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=fold_to_last | COMPLEXITY=2 | LINES=8

```rust
// Tests the ability of the compiler to recognize that only the last slice item is needed
// based on issue #106288
#[bench]
fn fold_to_last(b: &mut Bencher) {
    let slice: &[i32] = &[0; 1024];
    b.iter(|| black_box(slice).iter().fold(None, |_, r| Some(NonNull::from(r))));
}
```

---
*Generated by AST tracing system*
