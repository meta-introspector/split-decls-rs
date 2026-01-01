# AST Trace: ../rust/library/portable-simd/crates/core_simd/src/to_bytes.rs

Generated 19 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=4

```rust
use crate::simd::{
    LaneCount, Simd, SimdElement, SupportedLaneCount,
    num::{SimdFloat, SimdInt, SimdUint},
};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=6

```rust
mod sealed {
    use super::*;
    pub trait Sealed {}
    impl<T: SimdElement, const N: usize> Sealed for Simd<T, N> where LaneCount<N>: SupportedLaneCount {}
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=to_ne_bytes | COMPLEXITY=4 | LINES=36

```rust
use sealed::Sealed;

/// Converts SIMD vectors to vectors of bytes
pub trait ToBytes: Sealed {
    /// This type, reinterpreted as bytes.
    type Bytes: Copy
        + Unpin
        + Send
        + Sync
        + AsRef<[u8]>
        + AsMut<[u8]>
        + SimdUint<Scalar = u8>
        + 'static;

    /// Returns the memory representation of this integer as a byte array in native byte
    /// order.
    fn to_ne_bytes(self) -> Self::Bytes;

    /// Returns the memory representation of this integer as a byte array in big-endian
    /// (network) byte order.
    fn to_be_bytes(self) -> Self::Bytes;

    /// Returns the memory representation of this integer as a byte array in little-endian
    /// byte order.
    fn to_le_bytes(self) -> Self::Bytes;

    /// Creates a native endian integer value from its memory representation as a byte array
    /// in native endianness.
    fn from_ne_bytes(bytes: Self::Bytes) -> Self;

    /// Creates an integer value from its representation as a byte array in big endian.
    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    /// Creates an integer value from its representation as a byte array in little endian.
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=13 | LINES=6

```rust
macro_rules! swap_bytes {
    { f32, $x:expr } => { Simd::from_bits($x.to_bits().swap_bytes()) };
    { f64, $x:expr } => { Simd::from_bits($x.to_bits().swap_bytes()) };
    { $ty:ty, $x:expr } => { $x.swap_bytes() }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=to_ne_bytes | COMPLEXITY=68 | LINES=72

```rust
macro_rules! impl_to_bytes {
    { $ty:tt, 1  } => { impl_to_bytes! { $ty, 1  * [1, 2, 4, 8, 16, 32, 64] } };
    { $ty:tt, 2  } => { impl_to_bytes! { $ty, 2  * [1, 2, 4, 8, 16, 32] } };
    { $ty:tt, 4  } => { impl_to_bytes! { $ty, 4  * [1, 2, 4, 8, 16] } };
    { $ty:tt, 8  } => { impl_to_bytes! { $ty, 8  * [1, 2, 4, 8] } };
    { $ty:tt, 16 } => { impl_to_bytes! { $ty, 16 * [1, 2, 4] } };
    { $ty:tt, 32 } => { impl_to_bytes! { $ty, 32 * [1, 2] } };
    { $ty:tt, 64 } => { impl_to_bytes! { $ty, 64 * [1] } };

    { $ty:tt, $size:literal * [$($elems:literal),*] } => {
        $(
        impl ToBytes for Simd<$ty, $elems> {
            type Bytes = Simd<u8, { $size * $elems }>;

            #[inline]
            fn to_ne_bytes(self) -> Self::Bytes {
                // Safety: transmuting between vectors is safe
                unsafe {
                    #![allow(clippy::useless_transmute)]
                    core::mem::transmute(self)
                }
            }

            #[inline]
            fn to_be_bytes(mut self) -> Self::Bytes {
                if !cfg!(target_endian = "big") {
                    self = swap_bytes!($ty, self);
                }
                self.to_ne_bytes()
            }

            #[inline]
            fn to_le_bytes(mut self) -> Self::Bytes {
                if !cfg!(target_endian = "little") {
                    self = swap_bytes!($ty, self);
                }
                self.to_ne_bytes()
            }

            #[inline]
            fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                // Safety: transmuting between vectors is safe
                unsafe {
                    #![allow(clippy::useless_transmute)]
                    core::mem::transmute(bytes)
                }
            }

            #[inline]
            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                let ret = Self::from_ne_bytes(bytes);
                if cfg!(target_endian = "big") {
                    ret
                } else {
                    swap_bytes!($ty, ret)
                }
            }

            #[inline]
            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                let ret = Self::from_ne_bytes(bytes);
                if cfg!(target_endian = "little") {
                    ret
                } else {
                    swap_bytes!($ty, ret)
                }
            }
        }
        )*
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
impl_to_bytes! { u8, 1 }
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_to_bytes! { u16, 2 }
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_to_bytes! { u32, 4 }
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_to_bytes! { u64, 8 }
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[cfg(target_pointer_width = "32")]
impl_to_bytes! { usize, 4 }
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[cfg(target_pointer_width = "64")]
impl_to_bytes! { usize, 8 }
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
impl_to_bytes! { i8, 1 }
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_to_bytes! { i16, 2 }
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_to_bytes! { i32, 4 }
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_to_bytes! { i64, 8 }
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[cfg(target_pointer_width = "32")]
impl_to_bytes! { isize, 4 }
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[cfg(target_pointer_width = "64")]
impl_to_bytes! { isize, 8 }
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
impl_to_bytes! { f32, 4 }
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
impl_to_bytes! { f64, 8 }
```

---
*Generated by AST tracing system*
