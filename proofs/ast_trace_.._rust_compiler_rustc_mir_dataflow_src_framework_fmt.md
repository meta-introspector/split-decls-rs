# AST Trace: ../rust/compiler/rustc_mir_dataflow/src/framework/fmt.rs

Generated 15 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
//! Custom formatting traits used when outputting Graphviz diagrams with the results of a dataflow
//! analysis.

use std::fmt;

use rustc_index::Idx;
use rustc_index::bit_set::{ChunkedBitSet, DenseBitSet, MixedBitSet};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=fmt_with | COMPLEXITY=19 | LINES=34

```rust
use super::lattice::MaybeReachable;

/// An extension to `fmt::Debug` for data that can be better printed with some auxiliary data `C`.
pub trait DebugWithContext<C>: Eq + fmt::Debug {
    fn fmt_with(&self, _ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }

    /// Print the difference between `self` and `old`.
    ///
    /// This should print nothing if `self == old`.
    ///
    /// `+` and `-` are typically used to indicate differences. However, these characters are
    /// fairly common and may be needed to print a types representation. If using them to indicate
    /// a diff, prefix them with the "Unit Separator"  control character (␟  U+001F).
    fn fmt_diff_with(&self, old: &Self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == old {
            return Ok(());
        }

        write!(f, "\u{001f}+")?;
        self.fmt_with(ctxt, f)?;

        if f.alternate() {
            write!(f, "\n")?;
        } else {
            write!(f, "\t")?;
        }

        write!(f, "\u{001f}-")?;
        old.fmt_with(ctxt, f)
    }
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=DebugWithAdapter | COMPLEXITY=2 | LINES=6

```rust
/// Implements `fmt::Debug` by deferring to `<T as DebugWithContext<C>>::fmt_with`.
pub struct DebugWithAdapter<'a, T, C> {
    pub this: T,
    pub ctxt: &'a C,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=9

```rust
impl<T, C> fmt::Debug for DebugWithAdapter<'_, T, C>
where
    T: DebugWithContext<C>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.this.fmt_with(self.ctxt, f)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=DebugDiffWithAdapter | COMPLEXITY=2 | LINES=7

```rust
/// Implements `fmt::Debug` by deferring to `<T as DebugWithContext<C>>::fmt_diff_with`.
pub struct DebugDiffWithAdapter<'a, T, C> {
    pub new: T,
    pub old: T,
    pub ctxt: &'a C,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=9

```rust
impl<T, C> fmt::Debug for DebugDiffWithAdapter<'_, T, C>
where
    T: DebugWithContext<C>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.new.fmt_diff_with(&self.old, self.ctxt, f)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=fmt_with | COMPLEXITY=15 | LINES=29

```rust
// Impls

impl<T, C> DebugWithContext<C> for DenseBitSet<T>
where
    T: Idx + DebugWithContext<C>,
{
    fn fmt_with(&self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter().map(|i| DebugWithAdapter { this: i, ctxt })).finish()
    }

    fn fmt_diff_with(&self, old: &Self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.domain_size();
        assert_eq!(size, old.domain_size());

        let mut set_in_self = MixedBitSet::new_empty(size);
        let mut cleared_in_self = MixedBitSet::new_empty(size);

        for i in (0..size).map(T::new) {
            match (self.contains(i), old.contains(i)) {
                (true, false) => set_in_self.insert(i),
                (false, true) => cleared_in_self.insert(i),
                _ => continue,
            };
        }

        fmt_diff(&set_in_self, &cleared_in_self, ctxt, f)
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=fmt_with | COMPLEXITY=15 | LINES=27

```rust
impl<T, C> DebugWithContext<C> for ChunkedBitSet<T>
where
    T: Idx + DebugWithContext<C>,
{
    fn fmt_with(&self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter().map(|i| DebugWithAdapter { this: i, ctxt })).finish()
    }

    fn fmt_diff_with(&self, old: &Self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.domain_size();
        assert_eq!(size, old.domain_size());

        let mut set_in_self = MixedBitSet::new_empty(size);
        let mut cleared_in_self = MixedBitSet::new_empty(size);

        for i in (0..size).map(T::new) {
            match (self.contains(i), old.contains(i)) {
                (true, false) => set_in_self.insert(i),
                (false, true) => cleared_in_self.insert(i),
                _ => continue,
            };
        }

        fmt_diff(&set_in_self, &cleared_in_self, ctxt, f)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=fmt_with | COMPLEXITY=15 | LINES=20

```rust
impl<T, C> DebugWithContext<C> for MixedBitSet<T>
where
    T: Idx + DebugWithContext<C>,
{
    fn fmt_with(&self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MixedBitSet::Small(set) => set.fmt_with(ctxt, f),
            MixedBitSet::Large(set) => set.fmt_with(ctxt, f),
        }
    }

    fn fmt_diff_with(&self, old: &Self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self, old) {
            (MixedBitSet::Small(set), MixedBitSet::Small(old)) => set.fmt_diff_with(old, ctxt, f),
            (MixedBitSet::Large(set), MixedBitSet::Large(old)) => set.fmt_diff_with(old, ctxt, f),
            _ => panic!("MixedBitSet size mismatch"),
        }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=fmt_with | COMPLEXITY=22 | LINES=31

```rust
impl<S, C> DebugWithContext<C> for MaybeReachable<S>
where
    S: DebugWithContext<C>,
{
    fn fmt_with(&self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaybeReachable::Unreachable => {
                write!(f, "unreachable")
            }
            MaybeReachable::Reachable(set) => set.fmt_with(ctxt, f),
        }
    }

    fn fmt_diff_with(&self, old: &Self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self, old) {
            (MaybeReachable::Unreachable, MaybeReachable::Unreachable) => Ok(()),
            (MaybeReachable::Unreachable, MaybeReachable::Reachable(set)) => {
                write!(f, "\u{001f}+")?;
                set.fmt_with(ctxt, f)
            }
            (MaybeReachable::Reachable(set), MaybeReachable::Unreachable) => {
                write!(f, "\u{001f}-")?;
                set.fmt_with(ctxt, f)
            }
            (MaybeReachable::Reachable(this), MaybeReachable::Reachable(old)) => {
                this.fmt_diff_with(old, ctxt, f)
            }
        }
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=fmt_diff | COMPLEXITY=35 | LINES=48

```rust
fn fmt_diff<T, C>(
    inserted: &MixedBitSet<T>,
    removed: &MixedBitSet<T>,
    ctxt: &C,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    T: Idx + DebugWithContext<C>,
{
    let mut first = true;
    for idx in inserted.iter() {
        let delim = if first {
            "\u{001f}+"
        } else if f.alternate() {
            "\n\u{001f}+"
        } else {
            ", "
        };

        write!(f, "{delim}")?;
        idx.fmt_with(ctxt, f)?;
        first = false;
    }

    if !f.alternate() {
        first = true;
        if !inserted.is_empty() && !removed.is_empty() {
            write!(f, "\t")?;
        }
    }

    for idx in removed.iter() {
        let delim = if first {
            "\u{001f}-"
        } else if f.alternate() {
            "\n\u{001f}-"
        } else {
            ", "
        };

        write!(f, "{delim}")?;
        idx.fmt_with(ctxt, f)?;
        first = false;
    }

    Ok(())
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=fmt_with | COMPLEXITY=6 | LINES=13

```rust
impl<T, C> DebugWithContext<C> for &'_ T
where
    T: DebugWithContext<C>,
{
    fn fmt_with(&self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self).fmt_with(ctxt, f)
    }

    fn fmt_diff_with(&self, old: &Self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self).fmt_diff_with(*old, ctxt, f)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2

```rust
impl<C> DebugWithContext<C> for rustc_middle::mir::Local {}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1

```rust
impl<C> DebugWithContext<C> for crate::move_paths::InitIndex {}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=fmt_with | COMPLEXITY=6 | LINES=9

```rust
impl<'tcx, C> DebugWithContext<C> for crate::move_paths::MovePathIndex
where
    C: crate::move_paths::HasMoveData<'tcx>,
{
    fn fmt_with(&self, ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ctxt.move_data().move_paths[*self])
    }
}
```

---
*Generated by AST tracing system*
