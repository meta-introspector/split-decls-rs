# AST Trace: ../rust/compiler/rustc_arena/src/tests.rs

Generated 22 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=STRUCT | NAME=Point | COMPLEXITY=2 | LINES=14

```rust
extern crate test;
use std::cell::Cell;

use test::Bencher;

use super::TypedArena;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=clear | COMPLEXITY=15 | LINES=18

```rust
impl<T> TypedArena<T> {
    /// Clears the arena. Deallocates all but the longest chunk which may be reused.
    fn clear(&mut self) {
        unsafe {
            // Clear the last chunk, which is partially filled.
            let mut chunks_borrow = self.chunks.borrow_mut();
            if let Some(mut last_chunk) = chunks_borrow.last_mut() {
                self.clear_last_chunk(&mut last_chunk);
                let len = chunks_borrow.len();
                // If `T` is ZST, code below has no effect.
                for mut chunk in chunks_borrow.drain(..len - 1) {
                    chunk.destroy(chunk.entries);
                }
            }
        }
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=test_unused | COMPLEXITY=2 | LINES=6

```rust
#[test]
fn test_unused() {
    let arena: TypedArena<Point> = TypedArena::default();
    assert!(arena.chunks.borrow().is_empty());
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=test_arena_alloc_nested | COMPLEXITY=19 | LINES=37

```rust
#[test]
fn test_arena_alloc_nested() {
    struct Inner {
        value: u8,
    }
    struct Outer<'a> {
        inner: &'a Inner,
    }
    enum EI<'e> {
        I(Inner),
        O(Outer<'e>),
    }

    struct Wrap<'a>(TypedArena<EI<'a>>);

    impl<'a> Wrap<'a> {
        fn alloc_inner<F: Fn() -> Inner>(&self, f: F) -> &Inner {
            match self.0.alloc(EI::I(f())) {
                EI::I(i) => i,
                _ => panic!("mismatch"),
            }
        }
        fn alloc_outer<F: Fn() -> Outer<'a>>(&self, f: F) -> &Outer<'_> {
            match self.0.alloc(EI::O(f())) {
                EI::O(o) => o,
                _ => panic!("mismatch"),
            }
        }
    }

    let arena = Wrap(TypedArena::default());

    let result = arena.alloc_outer(|| Outer { inner: arena.alloc_inner(|| Inner { value: 10 }) });

    assert_eq!(result.inner.value, 10);
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=test_copy | COMPLEXITY=6 | LINES=12

```rust
#[test]
fn test_copy() {
    let arena = TypedArena::default();
    #[cfg(not(miri))]
    const N: usize = 100000;
    #[cfg(miri)]
    const N: usize = 1000;
    for _ in 0..N {
        arena.alloc(Point { x: 1, y: 2, z: 3 });
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=bench_copy | COMPLEXITY=3 | LINES=6

```rust
#[bench]
fn bench_copy(b: &mut Bencher) {
    let arena = TypedArena::default();
    b.iter(|| arena.alloc(Point { x: 1, y: 2, z: 3 }))
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=bench_copy_nonarena | COMPLEXITY=4 | LINES=7

```rust
#[bench]
fn bench_copy_nonarena(b: &mut Bencher) {
    b.iter(|| {
        let _: Box<_> = Box::new(Point { x: 1, y: 2, z: 3 });
    })
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=Noncopy | COMPLEXITY=2 | LINES=6

```rust
#[allow(dead_code)]
struct Noncopy {
    string: String,
    array: Vec<i32>,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=test_noncopy | COMPLEXITY=6 | LINES=12

```rust
#[test]
fn test_noncopy() {
    let arena = TypedArena::default();
    #[cfg(not(miri))]
    const N: usize = 100000;
    #[cfg(miri)]
    const N: usize = 1000;
    for _ in 0..N {
        arena.alloc(Noncopy { string: "hello world".to_string(), array: vec![1, 2, 3, 4, 5] });
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=test_typed_arena_zero_sized | COMPLEXITY=5 | LINES=12

```rust
#[test]
fn test_typed_arena_zero_sized() {
    let arena = TypedArena::default();
    #[cfg(not(miri))]
    const N: usize = 100000;
    #[cfg(miri)]
    const N: usize = 1000;
    for _ in 0..N {
        arena.alloc(());
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=test_typed_arena_clear | COMPLEXITY=9 | LINES=15

```rust
#[test]
fn test_typed_arena_clear() {
    let mut arena = TypedArena::default();
    for _ in 0..10 {
        arena.clear();
        #[cfg(not(miri))]
        const N: usize = 10000;
        #[cfg(miri)]
        const N: usize = 100;
        for _ in 0..N {
            arena.alloc(Point { x: 1, y: 2, z: 3 });
        }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=bench_typed_arena_clear | COMPLEXITY=4 | LINES=9

```rust
#[bench]
fn bench_typed_arena_clear(b: &mut Bencher) {
    let mut arena = TypedArena::default();
    b.iter(|| {
        arena.alloc(Point { x: 1, y: 2, z: 3 });
        arena.clear();
    })
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=bench_typed_arena_clear_100 | COMPLEXITY=7 | LINES=11

```rust
#[bench]
fn bench_typed_arena_clear_100(b: &mut Bencher) {
    let mut arena = TypedArena::default();
    b.iter(|| {
        for _ in 0..100 {
            arena.alloc(Point { x: 1, y: 2, z: 3 });
        }
        arena.clear();
    })
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=DropCounter | COMPLEXITY=2 | LINES=6

```rust
// Drop tests

struct DropCounter<'a> {
    count: &'a Cell<u32>,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=5 | LINES=6

```rust
impl Drop for DropCounter<'_> {
    fn drop(&mut self) {
        self.count.set(self.count.get() + 1);
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=test_typed_arena_drop_count | COMPLEXITY=7 | LINES=13

```rust
#[test]
fn test_typed_arena_drop_count() {
    let counter = Cell::new(0);
    {
        let arena: TypedArena<DropCounter<'_>> = TypedArena::default();
        for _ in 0..100 {
            // Allocate something with drop glue to make sure it doesn't leak.
            arena.alloc(DropCounter { count: &counter });
        }
    };
    assert_eq!(counter.get(), 100);
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=test_typed_arena_drop_on_clear | COMPLEXITY=9 | LINES=14

```rust
#[test]
fn test_typed_arena_drop_on_clear() {
    let counter = Cell::new(0);
    let mut arena: TypedArena<DropCounter<'_>> = TypedArena::default();
    for i in 0..10 {
        for _ in 0..100 {
            // Allocate something with drop glue to make sure it doesn't leak.
            arena.alloc(DropCounter { count: &counter });
        }
        arena.clear();
        assert_eq!(counter.get(), i * 100 + 100);
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
thread_local! {
    static DROP_COUNTER: Cell<u32> = Cell::new(0)
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=SmallDroppable; | COMPLEXITY=5 | LINES=8

```rust
struct SmallDroppable;

impl Drop for SmallDroppable {
    fn drop(&mut self) {
        DROP_COUNTER.with(|c| c.set(c.get() + 1));
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=test_typed_arena_drop_small_count | COMPLEXITY=6 | LINES=14

```rust
#[test]
fn test_typed_arena_drop_small_count() {
    DROP_COUNTER.with(|c| c.set(0));
    {
        let arena: TypedArena<SmallDroppable> = TypedArena::default();
        for _ in 0..100 {
            // Allocate something with drop glue to make sure it doesn't leak.
            arena.alloc(SmallDroppable);
        }
        // dropping
    };
    assert_eq!(DROP_COUNTER.with(|c| c.get()), 100);
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=bench_noncopy | COMPLEXITY=4 | LINES=8

```rust
#[bench]
fn bench_noncopy(b: &mut Bencher) {
    let arena = TypedArena::default();
    b.iter(|| {
        arena.alloc(Noncopy { string: "hello world".to_string(), array: vec![1, 2, 3, 4, 5] })
    })
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=bench_noncopy_nonarena | COMPLEXITY=4 | LINES=8

```rust
#[bench]
fn bench_noncopy_nonarena(b: &mut Bencher) {
    b.iter(|| {
        let _: Box<_> =
            Box::new(Noncopy { string: "hello world".to_string(), array: vec![1, 2, 3, 4, 5] });
    })
}
```

---
*Generated by AST tracing system*
