# AST Trace: ../rust/compiler/rustc_hashes/src/lib.rs

Generated 13 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=19

```rust
// rustc encodes a lot of hashes. If hashes are stored as `u64` or `u128`, a `derive(Encodable)`
// will apply varint encoding to the hashes, which is less efficient than directly encoding the 8
// or 16 bytes of the hash. And if that hash depends on the `StableCrateHash` (which most in rustc
// do), the varint encoding will make the number of bytes encoded fluctuate between compiler
// versions.
//
// The types in this module represent 64-bit or 128-bit hashes produced by a `StableHasher`.
// `Hash64` and `Hash128` expose some utility functions to encourage users to not extract the inner
// hash value as an integer type and accidentally apply varint encoding to it.
//
// In contrast with `Fingerprint`, users of these types cannot and should not attempt to construct
// and decompose these types into constituent pieces. The point of these types is only to
// connect the fact that they can only be produced by a `StableHasher` to their
// `Encode`/`Decode` impls.

use std::fmt;
use std::ops::BitXorAssign;

use rustc_stable_hash::{FromStableHash, SipHasher128Hash as StableHasherHash};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=STRUCT | NAME=Hash64 | COMPLEXITY=4 | LINES=6

```rust
/// A `u64` but encoded with a fixed size; for hashes this encoding is more compact than `u64`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Hash64 {
    inner: u64,
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=new | COMPLEXITY=8 | LINES=19

```rust
impl Hash64 {
    pub const ZERO: Hash64 = Hash64 { inner: 0 };

    #[inline]
    pub fn new(n: u64) -> Self {
        Self { inner: n }
    }

    #[inline]
    pub fn as_u64(self) -> u64 {
        self.inner
    }

    #[inline]
    pub fn wrapping_add(self, other: Self) -> Self {
        Self { inner: self.inner.wrapping_add(other.inner) }
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=bitxor_assign | COMPLEXITY=5 | LINES=7

```rust
impl BitXorAssign<u64> for Hash64 {
    #[inline]
    fn bitxor_assign(&mut self, rhs: u64) {
        self.inner ^= rhs;
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=from | COMPLEXITY=6 | LINES=9

```rust
impl FromStableHash for Hash64 {
    type Hash = StableHasherHash;

    #[inline]
    fn from(StableHasherHash([_0, __1]): Self::Hash) -> Self {
        Self { inner: _0 }
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Debug for Hash64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::LowerHex for Hash64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.inner, f)
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=Hash128 | COMPLEXITY=4 | LINES=6

```rust
/// A `u128` but encoded with a fixed size; for hashes this encoding is more compact than `u128`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Hash128 {
    inner: u128,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=hash | COMPLEXITY=5 | LINES=10

```rust
// We expect Hash128 to be well mixed. So there's no point in hashing both parts.
//
// This also allows using Hash128-containing types in UnHash-based hashmaps, which would otherwise
// debug_assert! that we're hashing more than a single u64.
impl std::hash::Hash for Hash128 {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        h.write_u64(self.truncate().as_u64());
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=new | COMPLEXITY=9 | LINES=22

```rust
impl Hash128 {
    #[inline]
    pub fn new(n: u128) -> Self {
        Self { inner: n }
    }

    #[inline]
    pub fn truncate(self) -> Hash64 {
        Hash64 { inner: self.inner as u64 }
    }

    #[inline]
    pub fn wrapping_add(self, other: Self) -> Self {
        Self { inner: self.inner.wrapping_add(other.inner) }
    }

    #[inline]
    pub fn as_u128(self) -> u128 {
        self.inner
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=from | COMPLEXITY=6 | LINES=9

```rust
impl FromStableHash for Hash128 {
    type Hash = StableHasherHash;

    #[inline]
    fn from(StableHasherHash([_0, _1]): Self::Hash) -> Self {
        Self { inner: u128::from(_0) | (u128::from(_1) << 64) }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Debug for Hash128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::LowerHex for Hash128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.inner, f)
    }
}
```

---
*Generated by AST tracing system*
