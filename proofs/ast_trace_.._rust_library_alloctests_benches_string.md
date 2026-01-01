# AST Trace: ../rust/library/alloctests/benches/string.rs

Generated 18 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use std::iter::repeat;

use test::{Bencher, black_box};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=bench_with_capacity | COMPLEXITY=2 | LINES=5

```rust
#[bench]
fn bench_with_capacity(b: &mut Bencher) {
    b.iter(|| String::with_capacity(black_box(100)));
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=bench_push_str | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_push_str(b: &mut Bencher) {
    let s = "ศไทย中华Việt Nam; Mary had a little lamb, Little lamb";
    b.iter(|| {
        let mut r = String::new();
        black_box(&mut r).push_str(black_box(s));
        r
    });
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=bench_push_str_one_byte | COMPLEXITY=6 | LINES=14

```rust
const REPETITIONS: u64 = 10_000;

#[bench]
fn bench_push_str_one_byte(b: &mut Bencher) {
    b.bytes = REPETITIONS;
    b.iter(|| {
        let mut r = String::new();
        for _ in 0..REPETITIONS {
            black_box(&mut r).push_str(black_box("a"));
        }
        r
    });
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=bench_push_char_one_byte | COMPLEXITY=6 | LINES=12

```rust
#[bench]
fn bench_push_char_one_byte(b: &mut Bencher) {
    b.bytes = REPETITIONS;
    b.iter(|| {
        let mut r = String::new();
        for _ in 0..REPETITIONS {
            black_box(&mut r).push(black_box('a'));
        }
        r
    });
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=bench_push_char_two_bytes | COMPLEXITY=6 | LINES=12

```rust
#[bench]
fn bench_push_char_two_bytes(b: &mut Bencher) {
    b.bytes = REPETITIONS * 2;
    b.iter(|| {
        let mut r = String::new();
        for _ in 0..REPETITIONS {
            black_box(&mut r).push(black_box('â'));
        }
        r
    });
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=from_utf8_lossy_100_ascii | COMPLEXITY=2 | LINES=9

```rust
#[bench]
fn from_utf8_lossy_100_ascii(b: &mut Bencher) {
    let s = b"Hello there, the quick brown fox jumped over the lazy dog! \
              Lorem ipsum dolor sit amet, consectetur. ";

    assert_eq!(100, s.len());
    b.iter(|| String::from_utf8_lossy(black_box(s)));
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=from_utf8_lossy_100_multibyte | COMPLEXITY=2 | LINES=7

```rust
#[bench]
fn from_utf8_lossy_100_multibyte(b: &mut Bencher) {
    let s = "𐌀𐌖𐌋𐌄𐌑𐌉ปรدولة الكويتทศไทย中华𐍅𐌿𐌻𐍆𐌹𐌻𐌰".as_bytes();
    assert_eq!(100, s.len());
    b.iter(|| String::from_utf8_lossy(black_box(s)));
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=from_utf8_lossy_invalid | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn from_utf8_lossy_invalid(b: &mut Bencher) {
    let s = b"Hello\xC0\x80 There\xE6\x83 Goodbye";
    b.iter(|| String::from_utf8_lossy(black_box(s)));
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=from_utf8_lossy_100_invalid | COMPLEXITY=2 | LINES=6

```rust
#[bench]
fn from_utf8_lossy_100_invalid(b: &mut Bencher) {
    let s = repeat(0xf5).take(100).collect::<Vec<_>>();
    b.iter(|| String::from_utf8_lossy(black_box(&s)));
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=bench_exact_size_shrink_to_fit | COMPLEXITY=4 | LINES=16

```rust
#[bench]
fn bench_exact_size_shrink_to_fit(b: &mut Bencher) {
    let s = "Hello there, the quick brown fox jumped over the lazy dog! \
             Lorem ipsum dolor sit amet, consectetur. ";
    // ensure our operation produces an exact-size string before we benchmark it
    let mut r = String::with_capacity(s.len());
    r.push_str(s);
    assert_eq!(r.len(), r.capacity());
    b.iter(|| {
        let mut r = String::with_capacity(black_box(s.len()));
        r.push_str(black_box(s));
        r.shrink_to_fit();
        r
    });
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=bench_from_str | COMPLEXITY=2 | LINES=7

```rust
#[bench]
fn bench_from_str(b: &mut Bencher) {
    let s = "Hello there, the quick brown fox jumped over the lazy dog! \
             Lorem ipsum dolor sit amet, consectetur. ";
    b.iter(|| String::from(black_box(s)))
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=bench_from | COMPLEXITY=2 | LINES=7

```rust
#[bench]
fn bench_from(b: &mut Bencher) {
    let s = "Hello there, the quick brown fox jumped over the lazy dog! \
             Lorem ipsum dolor sit amet, consectetur. ";
    b.iter(|| String::from(black_box(s)))
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=bench_to_string | COMPLEXITY=2 | LINES=7

```rust
#[bench]
fn bench_to_string(b: &mut Bencher) {
    let s = "Hello there, the quick brown fox jumped over the lazy dog! \
             Lorem ipsum dolor sit amet, consectetur. ";
    b.iter(|| black_box(s).to_string())
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=bench_insert_char_short | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_insert_char_short(b: &mut Bencher) {
    let s = "Hello, World!";
    b.iter(|| {
        let mut x = String::from(s);
        black_box(&mut x).insert(black_box(6), black_box(' '));
        x
    })
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=bench_insert_char_long | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_insert_char_long(b: &mut Bencher) {
    let s = "Hello, World!";
    b.iter(|| {
        let mut x = String::from(s);
        black_box(&mut x).insert(black_box(6), black_box('❤'));
        x
    })
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=bench_insert_str_short | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_insert_str_short(b: &mut Bencher) {
    let s = "Hello, World!";
    b.iter(|| {
        let mut x = String::from(s);
        black_box(&mut x).insert_str(black_box(6), black_box(" "));
        x
    })
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=bench_insert_str_long | COMPLEXITY=3 | LINES=10

```rust
#[bench]
fn bench_insert_str_long(b: &mut Bencher) {
    let s = "Hello, World!";
    b.iter(|| {
        let mut x = String::from(s);
        black_box(&mut x).insert_str(black_box(6), black_box(" rustic "));
        x
    })
}
```

---
*Generated by AST tracing system*
