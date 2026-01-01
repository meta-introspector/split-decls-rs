# AST Trace: ../rust/library/alloctests/benches/vec.rs

Generated 118 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use std::iter::repeat;

use rand::RngCore;
use test::{Bencher, black_box};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=bench_new | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_new(b: &mut Bencher) {
    b.iter(|| Vec::<u32>::new())
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=do_bench_with_capacity | COMPLEXITY=2 | LINES=6

```rust
fn do_bench_with_capacity(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| Vec::<u32>::with_capacity(src_len))
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=bench_with_capacity_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_with_capacity_0000(b: &mut Bencher) {
    do_bench_with_capacity(b, 0)
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=bench_with_capacity_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_with_capacity_0010(b: &mut Bencher) {
    do_bench_with_capacity(b, 10)
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=bench_with_capacity_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_with_capacity_0100(b: &mut Bencher) {
    do_bench_with_capacity(b, 100)
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=bench_with_capacity_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_with_capacity_1000(b: &mut Bencher) {
    do_bench_with_capacity(b, 1000)
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=do_bench_from_fn | COMPLEXITY=2 | LINES=6

```rust
fn do_bench_from_fn(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| (0..src_len).collect::<Vec<_>>())
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=bench_from_fn_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_fn_0000(b: &mut Bencher) {
    do_bench_from_fn(b, 0)
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=bench_from_fn_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_fn_0010(b: &mut Bencher) {
    do_bench_from_fn(b, 10)
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=bench_from_fn_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_fn_0100(b: &mut Bencher) {
    do_bench_from_fn(b, 100)
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=bench_from_fn_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_fn_1000(b: &mut Bencher) {
    do_bench_from_fn(b, 1000)
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=do_bench_from_elem | COMPLEXITY=2 | LINES=6

```rust
fn do_bench_from_elem(b: &mut Bencher, src_len: usize) {
    b.bytes = src_len as u64;

    b.iter(|| repeat(5).take(src_len).collect::<Vec<usize>>())
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=bench_from_elem_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_elem_0000(b: &mut Bencher) {
    do_bench_from_elem(b, 0)
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=bench_from_elem_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_elem_0010(b: &mut Bencher) {
    do_bench_from_elem(b, 10)
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=bench_from_elem_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_elem_0100(b: &mut Bencher) {
    do_bench_from_elem(b, 100)
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=bench_from_elem_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_elem_1000(b: &mut Bencher) {
    do_bench_from_elem(b, 1000)
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=do_bench_from_slice | COMPLEXITY=2 | LINES=8

```rust
fn do_bench_from_slice(b: &mut Bencher, src_len: usize) {
    let src: Vec<_> = FromIterator::from_iter(0..src_len);

    b.bytes = src_len as u64;

    b.iter(|| src.as_slice().to_vec());
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=bench_from_slice_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_slice_0000(b: &mut Bencher) {
    do_bench_from_slice(b, 0)
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=bench_from_slice_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_slice_0010(b: &mut Bencher) {
    do_bench_from_slice(b, 10)
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=bench_from_slice_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_slice_0100(b: &mut Bencher) {
    do_bench_from_slice(b, 100)
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=bench_from_slice_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_slice_1000(b: &mut Bencher) {
    do_bench_from_slice(b, 1000)
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=do_bench_from_iter | COMPLEXITY=3 | LINES=11

```rust
fn do_bench_from_iter(b: &mut Bencher, src_len: usize) {
    let src: Vec<_> = FromIterator::from_iter(0..src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let dst: Vec<_> = FromIterator::from_iter(src.iter().cloned());
        dst
    });
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=bench_from_iter_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_iter_0000(b: &mut Bencher) {
    do_bench_from_iter(b, 0)
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=bench_from_iter_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_iter_0010(b: &mut Bencher) {
    do_bench_from_iter(b, 10)
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=bench_from_iter_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_iter_0100(b: &mut Bencher) {
    do_bench_from_iter(b, 100)
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=bench_from_iter_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_from_iter_1000(b: &mut Bencher) {
    do_bench_from_iter(b, 1000)
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=do_bench_extend | COMPLEXITY=3 | LINES=13

```rust
fn do_bench_extend(b: &mut Bencher, dst_len: usize, src_len: usize) {
    let dst: Vec<_> = FromIterator::from_iter(0..dst_len);
    let src: Vec<_> = FromIterator::from_iter(dst_len..dst_len + src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let mut dst = dst.clone();
        dst.extend(src.clone());
        dst
    });
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=bench_extend_0000_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_0000_0000(b: &mut Bencher) {
    do_bench_extend(b, 0, 0)
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=bench_extend_0000_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_0000_0010(b: &mut Bencher) {
    do_bench_extend(b, 0, 10)
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=bench_extend_0000_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_0000_0100(b: &mut Bencher) {
    do_bench_extend(b, 0, 100)
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=bench_extend_0000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_0000_1000(b: &mut Bencher) {
    do_bench_extend(b, 0, 1000)
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=bench_extend_0010_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_0010_0010(b: &mut Bencher) {
    do_bench_extend(b, 10, 10)
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=bench_extend_0100_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_0100_0100(b: &mut Bencher) {
    do_bench_extend(b, 100, 100)
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=bench_extend_1000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_1000_1000(b: &mut Bencher) {
    do_bench_extend(b, 1000, 1000)
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=do_bench_extend_from_slice | COMPLEXITY=3 | LINES=13

```rust
fn do_bench_extend_from_slice(b: &mut Bencher, dst_len: usize, src_len: usize) {
    let dst: Vec<_> = FromIterator::from_iter(0..dst_len);
    let src: Vec<_> = FromIterator::from_iter(dst_len..dst_len + src_len);

    b.bytes = src_len as u64;

    b.iter(|| {
        let mut dst = dst.clone();
        dst.extend_from_slice(&src);
        dst
    });
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=bench_extend_recycle | COMPLEXITY=3 | LINES=14

```rust
#[bench]
fn bench_extend_recycle(b: &mut Bencher) {
    let mut data = vec![0; 1000];

    b.iter(|| {
        let tmp = std::mem::take(&mut data);
        let mut to_extend = black_box(Vec::new());
        to_extend.extend(tmp.into_iter());
        data = black_box(to_extend);
    });

    black_box(data);
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=bench_extend_from_slice_0000_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_from_slice_0000_0000(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 0)
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=bench_extend_from_slice_0000_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_from_slice_0000_0010(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 10)
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=bench_extend_from_slice_0000_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_from_slice_0000_0100(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 100)
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=bench_extend_from_slice_0000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_from_slice_0000_1000(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 0, 1000)
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=bench_extend_from_slice_0010_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_from_slice_0010_0010(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 10, 10)
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=bench_extend_from_slice_0100_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_from_slice_0100_0100(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 100, 100)
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=bench_extend_from_slice_1000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_extend_from_slice_1000_1000(b: &mut Bencher) {
    do_bench_extend_from_slice(b, 1000, 1000)
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=do_bench_clone | COMPLEXITY=2 | LINES=8

```rust
fn do_bench_clone(b: &mut Bencher, src_len: usize) {
    let src: Vec<usize> = FromIterator::from_iter(0..src_len);

    b.bytes = src_len as u64;

    b.iter(|| src.clone());
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=bench_clone_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_0000(b: &mut Bencher) {
    do_bench_clone(b, 0)
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=bench_clone_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_0010(b: &mut Bencher) {
    do_bench_clone(b, 10)
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=bench_clone_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_0100(b: &mut Bencher) {
    do_bench_clone(b, 100)
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=bench_clone_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_1000(b: &mut Bencher) {
    do_bench_clone(b, 1000)
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=do_bench_clone_from | COMPLEXITY=6 | LINES=17

```rust
fn do_bench_clone_from(b: &mut Bencher, times: usize, dst_len: usize, src_len: usize) {
    let dst: Vec<_> = FromIterator::from_iter(0..src_len);
    let src: Vec<_> = FromIterator::from_iter(dst_len..dst_len + src_len);

    b.bytes = (times * src_len) as u64;

    b.iter(|| {
        let mut dst = dst.clone();

        for _ in 0..times {
            dst.clone_from(&src);
            dst = black_box(dst);
        }
        dst
    });
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=bench_clone_from_01_0000_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0000_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 0)
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=bench_clone_from_01_0000_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0000_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 10)
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=bench_clone_from_01_0000_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 100)
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=bench_clone_from_01_0000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 0, 1000)
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=bench_clone_from_01_0010_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0010_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 10, 10)
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=bench_clone_from_01_0100_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0100_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 100, 100)
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=FUNCTION | NAME=bench_clone_from_01_1000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_1000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 1000, 1000)
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=bench_clone_from_01_0010_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0010_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 10, 100)
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=bench_clone_from_01_0100_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0100_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 100, 1000)
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=bench_clone_from_01_0010_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0010_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 10, 0)
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=FUNCTION | NAME=bench_clone_from_01_0100_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_0100_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 100, 10)
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=FUNCTION | NAME=bench_clone_from_01_1000_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_01_1000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 1, 1000, 100)
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=FUNCTION | NAME=bench_clone_from_10_0000_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0000_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 0)
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=FUNCTION | NAME=bench_clone_from_10_0000_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0000_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 10)
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=FUNCTION | NAME=bench_clone_from_10_0000_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 100)
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=FUNCTION | NAME=bench_clone_from_10_0000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 0, 1000)
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=FUNCTION | NAME=bench_clone_from_10_0010_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0010_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 10, 10)
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=FUNCTION | NAME=bench_clone_from_10_0100_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0100_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 100, 100)
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=FUNCTION | NAME=bench_clone_from_10_1000_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_1000_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 1000, 1000)
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=FUNCTION | NAME=bench_clone_from_10_0010_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0010_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 10, 100)
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=FUNCTION | NAME=bench_clone_from_10_0100_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0100_1000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 100, 1000)
}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=FUNCTION | NAME=bench_clone_from_10_0010_0000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0010_0000(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 10, 0)
}
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=FUNCTION | NAME=bench_clone_from_10_0100_0010 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_0100_0010(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 100, 10)
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=FUNCTION | NAME=bench_clone_from_10_1000_0100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_clone_from_10_1000_0100(b: &mut Bencher) {
    do_bench_clone_from(b, 10, 1000, 100)
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=FUNCTION | NAME=$fname | COMPLEXITY=11 | LINES=17

```rust
macro_rules! bench_in_place {
    ($($fname:ident, $type:ty, $count:expr, $init:expr);*) => {
        $(
            #[bench]
            fn $fname(b: &mut Bencher) {
                b.iter(|| {
                    let src: Vec<$type> = black_box(vec![$init; $count]);
                    src.into_iter()
                        .enumerate()
                        .map(|(idx, e)| idx as $type ^ e)
                        .collect::<Vec<$type>>()
                });
            }
        )+
    };
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=FUNCTION | NAME=bench_in_place_recycle | COMPLEXITY=5 | LINES=37

```rust
bench_in_place![
    bench_in_place_xxu8_0010_i0,   u8,   10, 0;
    bench_in_place_xxu8_0100_i0,   u8,  100, 0;
    bench_in_place_xxu8_1000_i0,   u8, 1000, 0;
    bench_in_place_xxu8_0010_i1,   u8,   10, 1;
    bench_in_place_xxu8_0100_i1,   u8,  100, 1;
    bench_in_place_xxu8_1000_i1,   u8, 1000, 1;
    bench_in_place_xu32_0010_i0,  u32,   10, 0;
    bench_in_place_xu32_0100_i0,  u32,  100, 0;
    bench_in_place_xu32_1000_i0,  u32, 1000, 0;
    bench_in_place_xu32_0010_i1,  u32,   10, 1;
    bench_in_place_xu32_0100_i1,  u32,  100, 1;
    bench_in_place_xu32_1000_i1,  u32, 1000, 1;
    bench_in_place_u128_0010_i0, u128,   10, 0;
    bench_in_place_u128_0100_i0, u128,  100, 0;
    bench_in_place_u128_1000_i0, u128, 1000, 0;
    bench_in_place_u128_0010_i1, u128,   10, 1;
    bench_in_place_u128_0100_i1, u128,  100, 1;
    bench_in_place_u128_1000_i1, u128, 1000, 1
];

#[bench]
fn bench_in_place_recycle(b: &mut Bencher) {
    let mut data = vec![0; 1000];

    b.iter(|| {
        let tmp = std::mem::take(&mut data);
        data = black_box(
            tmp.into_iter()
                .enumerate()
                .map(|(idx, e)| idx.wrapping_add(e))
                .fuse()
                .collect::<Vec<usize>>(),
        );
    });
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=FUNCTION | NAME=bench_in_place_zip_recycle | COMPLEXITY=4 | LINES=19

```rust
#[bench]
fn bench_in_place_zip_recycle(b: &mut Bencher) {
    let mut data = vec![0u8; 1000];
    let mut rng = crate::bench_rng();
    let mut subst = vec![0u8; 1000];
    rng.fill_bytes(&mut subst[..]);

    b.iter(|| {
        let tmp = std::mem::take(&mut data);
        let mangled = tmp
            .into_iter()
            .zip(subst.iter().copied())
            .enumerate()
            .map(|(i, (d, s))| d.wrapping_add(i as u8) ^ s)
            .collect::<Vec<_>>();
        data = black_box(mangled);
    });
}
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=FUNCTION | NAME=bench_in_place_zip_iter_mut | COMPLEXITY=4 | LINES=16

```rust
#[bench]
fn bench_in_place_zip_iter_mut(b: &mut Bencher) {
    let mut data = vec![0u8; 256];
    let mut rng = crate::bench_rng();
    let mut subst = vec![0u8; 1000];
    rng.fill_bytes(&mut subst[..]);

    b.iter(|| {
        data.iter_mut().enumerate().for_each(|(i, d)| {
            *d = d.wrapping_add(i as u8) ^ subst[i];
        });
    });

    black_box(data);
}
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=FUNCTION | NAME=vec_cast | COMPLEXITY=7 | LINES=4

```rust
pub fn vec_cast<T, U>(input: Vec<T>) -> Vec<U> {
    input.into_iter().map(|e| unsafe { std::mem::transmute_copy(&e) }).collect()
}
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=FUNCTION | NAME=bench_transmute | COMPLEXITY=3 | LINES=12

```rust
#[bench]
fn bench_transmute(b: &mut Bencher) {
    let mut vec = vec![10u32; 100];
    b.bytes = 800; // 2 casts x 4 bytes x 100
    b.iter(|| {
        let v = std::mem::take(&mut vec);
        let v = black_box(vec_cast::<u32, i32>(v));
        let v = black_box(vec_cast::<i32, u32>(v));
        vec = v;
    });
}
```

## Block 81
**Metadata**: AST_ID=81 | TYPE=FUNCTION | NAME=Droppable(usize); | COMPLEXITY=5 | LINES=9

```rust
#[derive(Clone)]
struct Droppable(usize);

impl Drop for Droppable {
    fn drop(&mut self) {
        black_box(self);
    }
}
```

## Block 82
**Metadata**: AST_ID=82 | TYPE=FUNCTION | NAME=bench_in_place_collect_droppable | COMPLEXITY=3 | LINES=13

```rust
#[bench]
fn bench_in_place_collect_droppable(b: &mut Bencher) {
    let v: Vec<Droppable> = std::iter::repeat_with(|| Droppable(0)).take(1000).collect();
    b.iter(|| {
        v.clone()
            .into_iter()
            .skip(100)
            .enumerate()
            .map(|(i, e)| Droppable(i ^ e.0))
            .collect::<Vec<_>>()
    })
}
```

## Block 83
**Metadata**: AST_ID=83 | TYPE=FUNCTION | NAME=bench_chain_collect | COMPLEXITY=2 | LINES=13

```rust
// node.js gives out of memory error to use with length 1_100_000
#[cfg(target_os = "emscripten")]
const LEN: usize = 4096;

#[cfg(not(target_os = "emscripten"))]
const LEN: usize = 16384;

#[bench]
fn bench_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| data.iter().cloned().chain([1]).collect::<Vec<_>>());
}
```

## Block 84
**Metadata**: AST_ID=84 | TYPE=FUNCTION | NAME=bench_chain_chain_collect | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn bench_chain_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| data.iter().cloned().chain([1]).chain([2]).collect::<Vec<_>>());
}
```

## Block 85
**Metadata**: AST_ID=85 | TYPE=FUNCTION | NAME=bench_nest_chain_chain_collect | COMPLEXITY=3 | LINES=8

```rust
#[bench]
fn bench_nest_chain_chain_collect(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        data.iter().cloned().chain([1].iter().chain([2].iter()).cloned()).collect::<Vec<_>>()
    });
}
```

## Block 86
**Metadata**: AST_ID=86 | TYPE=FUNCTION | NAME=bench_range_map_collect | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_range_map_collect(b: &mut Bencher) {
    b.iter(|| (0..LEN).map(|_| u32::default()).collect::<Vec<_>>());
}
```

## Block 87
**Metadata**: AST_ID=87 | TYPE=FUNCTION | NAME=bench_chain_extend_ref | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_chain_extend_ref(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::with_capacity(data.len() + 1);
        v.extend(data.iter().chain([1].iter()));
        v
    });
}
```

## Block 88
**Metadata**: AST_ID=88 | TYPE=FUNCTION | NAME=bench_chain_extend_value | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_chain_extend_value(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::with_capacity(data.len() + 1);
        v.extend(data.iter().cloned().chain(Some(1)));
        v
    });
}
```

## Block 89
**Metadata**: AST_ID=89 | TYPE=FUNCTION | NAME=bench_rev_1 | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_rev_1(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::new();
        v.extend(data.iter().rev());
        v
    });
}
```

## Block 90
**Metadata**: AST_ID=90 | TYPE=FUNCTION | NAME=bench_rev_2 | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_rev_2(b: &mut Bencher) {
    let data = black_box([0; LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::with_capacity(data.len());
        v.extend(data.iter().rev());
        v
    });
}
```

## Block 91
**Metadata**: AST_ID=91 | TYPE=FUNCTION | NAME=bench_map_regular | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_map_regular(b: &mut Bencher) {
    let data = black_box([(0, 0); LEN]);
    b.iter(|| {
        let mut v = Vec::<u32>::new();
        v.extend(data.iter().map(|t| t.1));
        v
    });
}
```

## Block 92
**Metadata**: AST_ID=92 | TYPE=FUNCTION | NAME=bench_map_fast | COMPLEXITY=11 | LINES=15

```rust
#[bench]
fn bench_map_fast(b: &mut Bencher) {
    let data = black_box([(0, 0); LEN]);
    b.iter(|| {
        let mut result: Vec<u32> = Vec::with_capacity(data.len());
        for i in 0..data.len() {
            unsafe {
                *result.as_mut_ptr().add(i) = data[i].0;
                result.set_len(i);
            }
        }
        result
    });
}
```

## Block 93
**Metadata**: AST_ID=93 | TYPE=FUNCTION | NAME=random_sorted_fill | COMPLEXITY=12 | LINES=20

```rust
fn random_sorted_fill(mut seed: u32, buf: &mut [u32]) {
    let mask = if buf.len() < 8192 {
        0xFF
    } else if buf.len() < 200_000 {
        0xFFFF
    } else {
        0xFFFF_FFFF
    };

    for item in buf.iter_mut() {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;

        *item = seed & mask;
    }

    buf.sort();
}
```

## Block 94
**Metadata**: AST_ID=94 | TYPE=FUNCTION | NAME=bench_dedup_slice_truncate | COMPLEXITY=9 | LINES=24

```rust
// Measures performance of slice dedup impl.
// This was used to justify separate implementation of dedup for Vec.
// This algorithm was used for Vecs prior to Rust 1.52.
fn bench_dedup_slice_truncate(b: &mut Bencher, sz: usize) {
    let mut template = vec![0u32; sz];
    b.bytes = size_of_val(template.as_slice()) as u64;
    random_sorted_fill(0x43, &mut template);

    let mut vec = template.clone();
    b.iter(|| {
        let vec = black_box(&mut vec);
        let len = {
            let (dedup, _) = vec.partition_dedup();
            dedup.len()
        };
        vec.truncate(len);

        black_box(vec.first());
        let vec = black_box(vec);
        vec.clear();
        vec.extend_from_slice(&template);
    });
}
```

## Block 95
**Metadata**: AST_ID=95 | TYPE=FUNCTION | NAME=bench_vec_dedup_random | COMPLEXITY=4 | LINES=17

```rust
// Measures performance of Vec::dedup on random data.
fn bench_vec_dedup_random(b: &mut Bencher, sz: usize) {
    let mut template = vec![0u32; sz];
    b.bytes = size_of_val(template.as_slice()) as u64;
    random_sorted_fill(0x43, &mut template);

    let mut vec = template.clone();
    b.iter(|| {
        let vec = black_box(&mut vec);
        vec.dedup();
        black_box(vec.first());
        let vec = black_box(vec);
        vec.clear();
        vec.extend_from_slice(&template);
    });
}
```

## Block 96
**Metadata**: AST_ID=96 | TYPE=FUNCTION | NAME=bench_vec_dedup_none | COMPLEXITY=5 | LINES=21

```rust
// Measures performance of Vec::dedup when there is no items removed
fn bench_vec_dedup_none(b: &mut Bencher, sz: usize) {
    let mut template = vec![0u32; sz];
    b.bytes = size_of_val(template.as_slice()) as u64;
    template.chunks_exact_mut(2).for_each(|w| {
        w[0] = black_box(0);
        w[1] = black_box(5);
    });

    let mut vec = template.clone();
    b.iter(|| {
        let vec = black_box(&mut vec);
        vec.dedup();
        black_box(vec.first());
        // Unlike other benches of `dedup`
        // this doesn't reinitialize vec
        // because we measure how efficient dedup is
        // when no memory written
    });
}
```

## Block 97
**Metadata**: AST_ID=97 | TYPE=FUNCTION | NAME=bench_vec_dedup_all | COMPLEXITY=5 | LINES=19

```rust
// Measures performance of Vec::dedup when there is all items removed
fn bench_vec_dedup_all(b: &mut Bencher, sz: usize) {
    let mut template = vec![0u32; sz];
    b.bytes = size_of_val(template.as_slice()) as u64;
    template.iter_mut().for_each(|w| {
        *w = black_box(0);
    });

    let mut vec = template.clone();
    b.iter(|| {
        let vec = black_box(&mut vec);
        vec.dedup();
        black_box(vec.first());
        let vec = black_box(vec);
        vec.clear();
        vec.extend_from_slice(&template);
    });
}
```

## Block 98
**Metadata**: AST_ID=98 | TYPE=FUNCTION | NAME=bench_dedup_slice_truncate_100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_slice_truncate_100(b: &mut Bencher) {
    bench_dedup_slice_truncate(b, 100);
}
```

## Block 99
**Metadata**: AST_ID=99 | TYPE=FUNCTION | NAME=bench_dedup_random_100 | COMPLEXITY=2 | LINES=4

```rust
#[bench]
fn bench_dedup_random_100(b: &mut Bencher) {
    bench_vec_dedup_random(b, 100);
}
```

## Block 100
**Metadata**: AST_ID=100 | TYPE=FUNCTION | NAME=bench_dedup_none_100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_none_100(b: &mut Bencher) {
    bench_vec_dedup_none(b, 100);
}
```

## Block 101
**Metadata**: AST_ID=101 | TYPE=FUNCTION | NAME=bench_dedup_all_100 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_all_100(b: &mut Bencher) {
    bench_vec_dedup_all(b, 100);
}
```

## Block 102
**Metadata**: AST_ID=102 | TYPE=FUNCTION | NAME=bench_dedup_slice_truncate_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_slice_truncate_1000(b: &mut Bencher) {
    bench_dedup_slice_truncate(b, 1000);
}
```

## Block 103
**Metadata**: AST_ID=103 | TYPE=FUNCTION | NAME=bench_dedup_random_1000 | COMPLEXITY=2 | LINES=4

```rust
#[bench]
fn bench_dedup_random_1000(b: &mut Bencher) {
    bench_vec_dedup_random(b, 1000);
}
```

## Block 104
**Metadata**: AST_ID=104 | TYPE=FUNCTION | NAME=bench_dedup_none_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_none_1000(b: &mut Bencher) {
    bench_vec_dedup_none(b, 1000);
}
```

## Block 105
**Metadata**: AST_ID=105 | TYPE=FUNCTION | NAME=bench_dedup_all_1000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_all_1000(b: &mut Bencher) {
    bench_vec_dedup_all(b, 1000);
}
```

## Block 106
**Metadata**: AST_ID=106 | TYPE=FUNCTION | NAME=bench_dedup_slice_truncate_10000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_slice_truncate_10000(b: &mut Bencher) {
    bench_dedup_slice_truncate(b, 10000);
}
```

## Block 107
**Metadata**: AST_ID=107 | TYPE=FUNCTION | NAME=bench_dedup_random_10000 | COMPLEXITY=2 | LINES=4

```rust
#[bench]
fn bench_dedup_random_10000(b: &mut Bencher) {
    bench_vec_dedup_random(b, 10000);
}
```

## Block 108
**Metadata**: AST_ID=108 | TYPE=FUNCTION | NAME=bench_dedup_none_10000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_none_10000(b: &mut Bencher) {
    bench_vec_dedup_none(b, 10000);
}
```

## Block 109
**Metadata**: AST_ID=109 | TYPE=FUNCTION | NAME=bench_dedup_all_10000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_all_10000(b: &mut Bencher) {
    bench_vec_dedup_all(b, 10000);
}
```

## Block 110
**Metadata**: AST_ID=110 | TYPE=FUNCTION | NAME=bench_dedup_slice_truncate_100000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_slice_truncate_100000(b: &mut Bencher) {
    bench_dedup_slice_truncate(b, 100000);
}
```

## Block 111
**Metadata**: AST_ID=111 | TYPE=FUNCTION | NAME=bench_dedup_random_100000 | COMPLEXITY=2 | LINES=4

```rust
#[bench]
fn bench_dedup_random_100000(b: &mut Bencher) {
    bench_vec_dedup_random(b, 100000);
}
```

## Block 112
**Metadata**: AST_ID=112 | TYPE=FUNCTION | NAME=bench_dedup_none_100000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_none_100000(b: &mut Bencher) {
    bench_vec_dedup_none(b, 100000);
}
```

## Block 113
**Metadata**: AST_ID=113 | TYPE=FUNCTION | NAME=bench_dedup_all_100000 | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_dedup_all_100000(b: &mut Bencher) {
    bench_vec_dedup_all(b, 100000);
}
```

## Block 114
**Metadata**: AST_ID=114 | TYPE=FUNCTION | NAME=bench_flat_map_collect | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn bench_flat_map_collect(b: &mut Bencher) {
    let v = vec![777u32; 500000];
    b.iter(|| v.iter().flat_map(|color| color.rotate_left(8).to_be_bytes()).collect::<Vec<_>>());
}
```

## Block 115
**Metadata**: AST_ID=115 | TYPE=FUNCTION | NAME=bench_retain_iter_100000 | COMPLEXITY=3 | LINES=13

```rust
/// Reference benchmark that `retain` has to compete with.
#[bench]
fn bench_retain_iter_100000(b: &mut Bencher) {
    let mut v = Vec::with_capacity(100000);

    b.iter(|| {
        let mut tmp = std::mem::take(&mut v);
        tmp.clear();
        tmp.extend(black_box(1..=100000));
        v = tmp.into_iter().filter(|x| x & 1 == 0).collect();
    });
}
```

## Block 116
**Metadata**: AST_ID=116 | TYPE=FUNCTION | NAME=bench_retain_100000 | COMPLEXITY=3 | LINES=11

```rust
#[bench]
fn bench_retain_100000(b: &mut Bencher) {
    let mut v = Vec::with_capacity(100000);

    b.iter(|| {
        v.clear();
        v.extend(black_box(1..=100000));
        v.retain(|x| x & 1 == 0)
    });
}
```

## Block 117
**Metadata**: AST_ID=117 | TYPE=FUNCTION | NAME=bench_retain_whole_100000 | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn bench_retain_whole_100000(b: &mut Bencher) {
    let mut v = black_box(vec![826u32; 100000]);
    b.iter(|| v.retain(|x| *x == 826u32));
}
```

## Block 118
**Metadata**: AST_ID=118 | TYPE=FUNCTION | NAME=bench_next_chunk | COMPLEXITY=9 | LINES=20

```rust
#[bench]
fn bench_next_chunk(b: &mut Bencher) {
    let v = vec![13u8; 2048];

    b.iter(|| {
        const CHUNK: usize = 8;

        let mut sum = [0u32; CHUNK];
        let mut iter = black_box(v.clone()).into_iter();

        while let Ok(chunk) = iter.next_chunk::<CHUNK>() {
            for i in 0..CHUNK {
                sum[i] += chunk[i] as u32;
            }
        }

        sum
    })
}
```

---
*Generated by AST tracing system*
