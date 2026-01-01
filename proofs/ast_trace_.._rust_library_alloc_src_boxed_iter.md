# AST Trace: ../rust/library/alloc/src/boxed/iter.rs

Generated 22 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use core::async_iter::AsyncIterator;
use core::iter::FusedIterator;
use core::pin::Pin;
use core::slice;
use core::task::{Context, Poll};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=next | COMPLEXITY=9 | LINES=27

```rust
use crate::alloc::Allocator;
#[cfg(not(no_global_oom_handling))]
use crate::borrow::Cow;
use crate::boxed::Box;
#[cfg(not(no_global_oom_handling))]
use crate::string::String;
use crate::vec;
#[cfg(not(no_global_oom_handling))]
use crate::vec::Vec;

#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator + ?Sized, A: Allocator> Iterator for Box<I, A> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth(n)
    }
    fn last(self) -> Option<I::Item> {
        BoxIter::last(self)
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=last | COMPLEXITY=2 | LINES=5

```rust
trait BoxIter {
    type Item;
    fn last(self) -> Option<Self::Item>;
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=some | COMPLEXITY=6 | LINES=12

```rust
impl<I: Iterator + ?Sized, A: Allocator> BoxIter for Box<I, A> {
    type Item = I::Item;
    default fn last(self) -> Option<I::Item> {
        #[inline]
        fn some<T>(_: Option<T>, x: T) -> Option<T> {
            Some(x)
        }

        self.fold(None, some)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=last | COMPLEXITY=7 | LINES=9

```rust
/// Specialization for sized `I`s that uses `I`s implementation of `last()`
/// instead of the default.
#[stable(feature = "rust1", since = "1.0.0")]
impl<I: Iterator, A: Allocator> BoxIter for Box<I, A> {
    fn last(self) -> Option<I::Item> {
        (*self).last()
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=next_back | COMPLEXITY=6 | LINES=10

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I: DoubleEndedIterator + ?Sized, A: Allocator> DoubleEndedIterator for Box<I, A> {
    fn next_back(&mut self) -> Option<I::Item> {
        (**self).next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth_back(n)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=len | COMPLEXITY=6 | LINES=9

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<I: ExactSizeIterator + ?Sized, A: Allocator> ExactSizeIterator for Box<I, A> {
    fn len(&self) -> usize {
        (**self).len()
    }
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3

```rust
#[stable(feature = "fused", since = "1.26.0")]
impl<I: FusedIterator + ?Sized, A: Allocator> FusedIterator for Box<I, A> {}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=poll_next | COMPLEXITY=6 | LINES=13

```rust
#[unstable(feature = "async_iterator", issue = "79024")]
impl<S: ?Sized + AsyncIterator + Unpin> AsyncIterator for Box<S> {
    type Item = S::Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut **self).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=6 | LINES=5

```rust
/// This implementation is required to make sure that the `Box<[I]>: IntoIterator`
/// implementation doesn't overlap with `IntoIterator for T where T: Iterator` blanket.
#[stable(feature = "boxed_slice_into_iter", since = "1.80.0")]
impl<I, A: Allocator> !Iterator for Box<[I], A> {}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=6 | LINES=5

```rust
/// This implementation is required to make sure that the `&Box<[I]>: IntoIterator`
/// implementation doesn't overlap with `IntoIterator for T where T: Iterator` blanket.
#[stable(feature = "boxed_slice_into_iter", since = "1.80.0")]
impl<'a, I, A: Allocator> !Iterator for &'a Box<[I], A> {}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=6 | LINES=5

```rust
/// This implementation is required to make sure that the `&mut Box<[I]>: IntoIterator`
/// implementation doesn't overlap with `IntoIterator for T where T: Iterator` blanket.
#[stable(feature = "boxed_slice_into_iter", since = "1.80.0")]
impl<'a, I, A: Allocator> !Iterator for &'a mut Box<[I], A> {}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=into_iter | COMPLEXITY=6 | LINES=12

```rust
// Note: the `#[rustc_skip_during_method_dispatch(boxed_slice)]` on `trait IntoIterator`
// hides this implementation from explicit `.into_iter()` calls on editions < 2024,
// so those calls will still resolve to the slice implementation, by reference.
#[stable(feature = "boxed_slice_into_iter", since = "1.80.0")]
impl<I, A: Allocator> IntoIterator for Box<[I], A> {
    type IntoIter = vec::IntoIter<I, A>;
    type Item = I;
    fn into_iter(self) -> vec::IntoIter<I, A> {
        self.into_vec().into_iter()
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=into_iter | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "boxed_slice_into_iter", since = "1.80.0")]
impl<'a, I, A: Allocator> IntoIterator for &'a Box<[I], A> {
    type IntoIter = slice::Iter<'a, I>;
    type Item = &'a I;
    fn into_iter(self) -> slice::Iter<'a, I> {
        self.iter()
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=into_iter | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "boxed_slice_into_iter", since = "1.80.0")]
impl<'a, I, A: Allocator> IntoIterator for &'a mut Box<[I], A> {
    type IntoIter = slice::IterMut<'a, I>;
    type Item = &'a mut I;
    fn into_iter(self) -> slice::IterMut<'a, I> {
        self.iter_mut()
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=8

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "boxed_slice_from_iter", since = "1.32.0")]
impl<I> FromIterator<I> for Box<[I]> {
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into_boxed_slice()
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=8

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "boxed_str_from_iter", since = "1.80.0")]
impl FromIterator<char> for Box<str> {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        String::from_iter(iter).into_boxed_str()
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=8

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "boxed_str_from_iter", since = "1.80.0")]
impl<'a> FromIterator<&'a char> for Box<str> {
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        String::from_iter(iter).into_boxed_str()
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=8

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "boxed_str_from_iter", since = "1.80.0")]
impl<'a> FromIterator<&'a str> for Box<str> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        String::from_iter(iter).into_boxed_str()
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=8

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "boxed_str_from_iter", since = "1.80.0")]
impl FromIterator<String> for Box<str> {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        String::from_iter(iter).into_boxed_str()
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=8

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "boxed_str_from_iter", since = "1.80.0")]
impl<A: Allocator> FromIterator<Box<str, A>> for Box<str> {
    fn from_iter<T: IntoIterator<Item = Box<str, A>>>(iter: T) -> Self {
        String::from_iter(iter).into_boxed_str()
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=8

```rust
#[cfg(not(no_global_oom_handling))]
#[stable(feature = "boxed_str_from_iter", since = "1.80.0")]
impl<'a> FromIterator<Cow<'a, str>> for Box<str> {
    fn from_iter<T: IntoIterator<Item = Cow<'a, str>>>(iter: T) -> Self {
        String::from_iter(iter).into_boxed_str()
    }
}
```

---
*Generated by AST tracing system*
