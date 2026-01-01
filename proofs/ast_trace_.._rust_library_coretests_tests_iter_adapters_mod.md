# AST Trace: ../rust/library/coretests/tests/iter/adapters/mod.rs

Generated 20 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=STRUCT | NAME=NonFused | COMPLEXITY=3 | LINES=35

```rust
mod array_chunks;
mod by_ref_sized;
mod chain;
mod cloned;
mod copied;
mod cycle;
mod enumerate;
mod filter;
mod filter_map;
mod flat_map;
mod flatten;
mod fuse;
mod inspect;
mod intersperse;
mod map;
mod map_windows;
mod peekable;
mod scan;
mod skip;
mod skip_while;
mod step_by;
mod take;
mod take_while;
mod zip;

use core::cell::Cell;

/// An iterator that panics whenever `next` or `next_back` is called
/// after `None` has already been returned. This does not violate
/// `Iterator`'s contract. Used to test that iterator adapters don't
/// poll their inner iterators after exhausting them.
pub struct NonFused<I> {
    iter: I,
    done: bool,
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6

```rust
impl<I> NonFused<I> {
    pub fn new(iter: I) -> Self {
        Self { iter, done: false }
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=next | COMPLEXITY=6 | LINES=15

```rust
impl<I> Iterator for NonFused<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        assert!(!self.done, "this iterator has already returned None");
        self.iter.next().or_else(|| {
            self.done = true;
            None
        })
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=6 | LINES=13

```rust
impl<I> DoubleEndedIterator for NonFused<I>
where
    I: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        assert!(!self.done, "this iterator has already returned None");
        self.iter.next_back().or_else(|| {
            self.done = true;
            None
        })
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=Unfuse | COMPLEXITY=2 | LINES=7

```rust
/// An iterator wrapper that panics whenever `next` or `next_back` is called
/// after `None` has been returned.
pub struct Unfuse<I> {
    iter: I,
    exhausted: bool,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=9

```rust
impl<I> Unfuse<I> {
    pub fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<IntoIter = I>,
    {
        Self { iter: iter.into_iter(), exhausted: false }
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=next | COMPLEXITY=5 | LINES=14

```rust
impl<I> Iterator for Unfuse<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        assert!(!self.exhausted);
        let next = self.iter.next();
        self.exhausted = next.is_none();
        next
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=5 | LINES=12

```rust
impl<I> DoubleEndedIterator for Unfuse<I>
where
    I: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        assert!(!self.exhausted);
        let next = self.iter.next_back();
        self.exhausted = next.is_none();
        next
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=Toggle | COMPLEXITY=2 | LINES=4

```rust
pub struct Toggle {
    is_empty: bool,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=next | COMPLEXITY=14 | LINES=19

```rust
impl Iterator for Toggle {
    type Item = ();

    // alternates between `None` and `Some(())`
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty {
            self.is_empty = false;
            None
        } else {
            self.is_empty = true;
            Some(())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_empty { (0, Some(0)) } else { (1, Some(1)) }
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=5 | LINES=6

```rust
impl DoubleEndedIterator for Toggle {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=CycleIter | COMPLEXITY=4 | LINES=8

```rust
/// This is an iterator that follows the Iterator contract,
/// but it is not fused. After having returned None once, it will start
/// producing elements if .next() is called again.
pub struct CycleIter<'a, T> {
    index: usize,
    data: &'a [T],
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6

```rust
impl<'a, T> CycleIter<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        Self { index: 0, data }
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=next | COMPLEXITY=5 | LINES=10

```rust
impl<'a, T> Iterator for CycleIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let elt = self.data.get(self.index);
        self.index += 1;
        self.index %= 1 + self.data.len();
        elt
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=CountClone(Cell | COMPLEXITY=3 | LINES=9

```rust
#[derive(Debug)]
struct CountClone(Cell<i32>);

impl CountClone {
    pub fn new() -> Self {
        Self(Cell::new(0))
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=5 | LINES=6

```rust
impl PartialEq<i32> for CountClone {
    fn eq(&self, rhs: &i32) -> bool {
        self.0.get() == *rhs
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=5 | LINES=9

```rust
impl Clone for CountClone {
    fn clone(&self) -> Self {
        let ret = CountClone(self.0.clone());
        let n = self.0.get();
        self.0.set(n + 1);
        ret
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=STRUCT | NAME=CountDrop | COMPLEXITY=2 | LINES=6

```rust
#[derive(Debug, Clone)]
struct CountDrop<'a> {
    dropped: bool,
    count: &'a Cell<usize>,
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6

```rust
impl<'a> CountDrop<'a> {
    pub fn new(count: &'a Cell<usize>) -> Self {
        Self { dropped: false, count }
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=8 | LINES=10

```rust
impl Drop for CountDrop<'_> {
    fn drop(&mut self) {
        if self.dropped {
            panic!("double drop");
        }
        self.dropped = true;
        self.count.set(self.count.get() + 1);
    }
}
```

---
*Generated by AST tracing system*
