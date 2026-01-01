# AST Trace: ../rust/library/compiler-builtins/builtins-test/tests/aeabi_memset.rs

Generated 13 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=8 | LINES=25

```rust
#![cfg(all(
    target_arch = "arm",
    not(any(target_env = "gnu", target_env = "musl")),
    target_os = "linux",
    feature = "mem"
))]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate compiler_builtins;

// test runner
extern crate utest_cortex_m_qemu;

// overrides `panic!`
#[macro_use]
extern crate utest_macros;

use core::mem;

macro_rules! panic {
    ($($tt:tt)*) => {
        upanic!($($tt)*);
    };
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=__aeabi_memset4 | COMPLEXITY=6 | LINES=5

```rust
// SAFETY: defined in  compiler-builtins
unsafe extern "aapcs" {
    fn __aeabi_memset4(dest: *mut u8, n: usize, c: u32);
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=Aligned | COMPLEXITY=2 | LINES=5

```rust
struct Aligned {
    array: [u8; 8],
    _alignment: [u32; 0],
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=9

```rust
impl Aligned {
    fn new(array: [u8; 8]) -> Self {
        Aligned {
            array: array,
            _alignment: [],
        }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=zero | COMPLEXITY=12 | LINES=21

```rust
#[test]
fn zero() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), 0, c) }

    assert_eq!(*xs, [0; 8]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), 0, c) }

    assert_eq!(*xs, [1; 8]);
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=one | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn one() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 1;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0, 0, 0, 0, 0, 0, 0]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 1, 1, 1, 1, 1, 1, 1]);
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=two | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn two() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 2;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0, 0, 0, 0, 0, 0]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 1, 1, 1, 1, 1, 1]);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=three | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn three() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 3;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0, 0, 0, 0, 0]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 1, 1, 1, 1, 1]);
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=four | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn four() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 4;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0, 0, 0, 0]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 1, 1, 1, 1]);
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=five | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn five() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 5;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 0, 0, 0]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 1, 1, 1]);
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=six | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn six() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 6;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0, 0]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 1, 1]);
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=seven | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn seven() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 7;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 1]);
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=eight | COMPLEXITY=13 | LINES=22

```rust
#[test]
fn eight() {
    let mut aligned = Aligned::new([0u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let n = 8;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef]);

    let mut aligned = Aligned::new([1u8; 8]);
    assert_eq!(mem::align_of_val(&aligned), 4);
    let xs = &mut aligned.array;
    let c = 0xdeadbeef;

    unsafe { __aeabi_memset4(xs.as_mut_ptr(), n, c) }

    assert_eq!(*xs, [0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef, 0xef]);
}
```

---
*Generated by AST tracing system*
