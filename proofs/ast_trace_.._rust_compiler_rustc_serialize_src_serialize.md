# AST Trace: ../rust/compiler/rustc_serialize/src/serialize.rs

Generated 80 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=4

```rust
//! Support code for encoding and decoding types.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use std::hash::{BuildHasher, Hash};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use std::marker::{PhantomData, PointeeSized};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use std::num::NonZero;
use std::path;
use std::rc::Rc;
use std::sync::Arc;

use rustc_hashes::{Hash64, Hash128};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use smallvec::{Array, SmallVec};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=emit_usize | COMPLEXITY=18 | LINES=74

```rust
use thin_vec::ThinVec;

/// A byte that [cannot occur in UTF8 sequences][utf8]. Used to mark the end of a string.
/// This way we can skip validation and still be relatively sure that deserialization
/// did not desynchronize.
///
/// [utf8]: https://en.wikipedia.org/w/index.php?title=UTF-8&oldid=1058865525#Codepage_layout
const STR_SENTINEL: u8 = 0xC1;

/// For byte strings there are no bytes that cannot occur. Just use this value
/// as a best-effort sentinel. There is no validation skipped so the potential
/// for badness is lower than in the `STR_SENTINEL` case.
const BYTE_STR_SENTINEL: u8 = 0xC2;

/// A note about error handling.
///
/// Encoders may be fallible, but in practice failure is rare and there are so
/// many nested calls that typical Rust error handling (via `Result` and `?`)
/// is pervasive and has non-trivial cost. Instead, impls of this trait must
/// implement a delayed error handling strategy. If a failure occurs, they
/// should record this internally, and all subsequent encoding operations can
/// be processed or ignored, whichever is appropriate. Then they should provide
/// a `finish` method that finishes up encoding. If the encoder is fallible,
/// `finish` should return a `Result` that indicates success or failure.
///
/// This current does not support `f32` nor `f64`, as they're not needed in any
/// serialized data structures. That could be changed, but consider whether it
/// really makes sense to store floating-point values at all.
/// (If you need it, revert <https://github.com/rust-lang/rust/pull/109984>.)
pub trait Encoder {
    fn emit_usize(&mut self, v: usize);
    fn emit_u128(&mut self, v: u128);
    fn emit_u64(&mut self, v: u64);
    fn emit_u32(&mut self, v: u32);
    fn emit_u16(&mut self, v: u16);
    fn emit_u8(&mut self, v: u8);

    fn emit_isize(&mut self, v: isize);
    fn emit_i128(&mut self, v: i128);
    fn emit_i64(&mut self, v: i64);
    fn emit_i32(&mut self, v: i32);
    fn emit_i16(&mut self, v: i16);

    #[inline]
    fn emit_i8(&mut self, v: i8) {
        self.emit_u8(v as u8);
    }

    #[inline]
    fn emit_bool(&mut self, v: bool) {
        self.emit_u8(if v { 1 } else { 0 });
    }

    #[inline]
    fn emit_char(&mut self, v: char) {
        self.emit_u32(v as u32);
    }

    #[inline]
    fn emit_str(&mut self, v: &str) {
        self.emit_usize(v.len());
        self.emit_raw_bytes(v.as_bytes());
        self.emit_u8(STR_SENTINEL);
    }

    #[inline]
    fn emit_byte_str(&mut self, v: &[u8]) {
        self.emit_usize(v.len());
        self.emit_raw_bytes(v);
        self.emit_u8(BYTE_STR_SENTINEL);
    }

    fn emit_raw_bytes(&mut self, s: &[u8]);
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=read_usize | COMPLEXITY=18 | LINES=66

```rust
// Note: all the methods in this trait are infallible, which may be surprising.
// They used to be fallible (i.e. return a `Result`) but many of the impls just
// panicked when something went wrong, and for the cases that didn't the
// top-level invocation would also just panic on failure. Switching to
// infallibility made things faster and lots of code a little simpler and more
// concise.
///
/// This current does not support `f32` nor `f64`, as they're not needed in any
/// serialized data structures. That could be changed, but consider whether it
/// really makes sense to store floating-point values at all.
/// (If you need it, revert <https://github.com/rust-lang/rust/pull/109984>.)
pub trait Decoder {
    fn read_usize(&mut self) -> usize;
    fn read_u128(&mut self) -> u128;
    fn read_u64(&mut self) -> u64;
    fn read_u32(&mut self) -> u32;
    fn read_u16(&mut self) -> u16;
    fn read_u8(&mut self) -> u8;

    fn read_isize(&mut self) -> isize;
    fn read_i128(&mut self) -> i128;
    fn read_i64(&mut self) -> i64;
    fn read_i32(&mut self) -> i32;
    fn read_i16(&mut self) -> i16;

    #[inline]
    fn read_i8(&mut self) -> i8 {
        self.read_u8() as i8
    }

    #[inline]
    fn read_bool(&mut self) -> bool {
        let value = self.read_u8();
        value != 0
    }

    #[inline]
    fn read_char(&mut self) -> char {
        let bits = self.read_u32();
        std::char::from_u32(bits).unwrap()
    }

    #[inline]
    fn read_str(&mut self) -> &str {
        let len = self.read_usize();
        let bytes = self.read_raw_bytes(len + 1);
        assert!(bytes[len] == STR_SENTINEL);
        // SAFETY: the presence of `STR_SENTINEL` gives us high (but not
        // perfect) confidence that the bytes we just read truly are UTF-8.
        unsafe { std::str::from_utf8_unchecked(&bytes[..len]) }
    }

    #[inline]
    fn read_byte_str(&mut self) -> &[u8] {
        let len = self.read_usize();
        let bytes = self.read_raw_bytes(len + 1);
        assert!(bytes[len] == BYTE_STR_SENTINEL);
        &bytes[..len]
    }

    fn read_raw_bytes(&mut self, len: usize) -> &[u8];

    fn peek_byte(&self) -> u8;
    fn position(&self) -> usize;
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=9 | LINES=15

```rust
/// Trait for types that can be serialized
///
/// This can be implemented using the `Encodable`, `TyEncodable` and
/// `MetadataEncodable` macros.
///
/// * `Encodable` should be used in crates that don't depend on
///   `rustc_middle`.
/// * `MetadataEncodable` is used in `rustc_metadata` for types that contain
///   `rustc_metadata::rmeta::Lazy`.
/// * `TyEncodable` should be used for types that are only serialized in crate
///   metadata or the incremental cache. This is most types in `rustc_middle`.
pub trait Encodable<S: Encoder>: PointeeSized {
    fn encode(&self, s: &mut S);
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=9 | LINES=15

```rust
/// Trait for types that can be deserialized
///
/// This can be implemented using the `Decodable`, `TyDecodable` and
/// `MetadataDecodable` macros.
///
/// * `Decodable` should be used in crates that don't depend on
///   `rustc_middle`.
/// * `MetadataDecodable` is used in `rustc_metadata` for types that contain
///   `rustc_metadata::rmeta::Lazy`.
/// * `TyDecodable` should be used for types that are only serialized in crate
///   metadata or the incremental cache. This is most types in `rustc_middle`.
pub trait Decodable<D: Decoder>: Sized {
    fn decode(d: &mut D) -> Self;
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=16 | LINES=18

```rust
macro_rules! direct_serialize_impls {
    ($($ty:ident $emit_method:ident $read_method:ident),*) => {
        $(
            impl<S: Encoder> Encodable<S> for $ty {
                fn encode(&self, s: &mut S) {
                    s.$emit_method(*self);
                }
            }

            impl<D: Decoder> Decodable<D> for $ty {
                fn decode(d: &mut D) -> $ty {
                    d.$read_method()
                }
            }
        )*
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=19

```rust
direct_serialize_impls! {
    usize emit_usize read_usize,
    u8 emit_u8 read_u8,
    u16 emit_u16 read_u16,
    u32 emit_u32 read_u32,
    u64 emit_u64 read_u64,
    u128 emit_u128 read_u128,

    isize emit_isize read_isize,
    i8 emit_i8 read_i8,
    i16 emit_i16 read_i16,
    i32 emit_i32 read_i32,
    i64 emit_i64 read_i64,
    i128 emit_i128 read_i128,

    bool emit_bool read_bool,
    char emit_char read_char
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=9

```rust
impl<S: Encoder, T: ?Sized + PointeeSized> Encodable<S> for &T
where
    T: Encodable<S>,
{
    fn encode(&self, s: &mut S) {
        (**self).encode(s)
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder> Encodable<S> for ! {
    fn encode(&self, _s: &mut S) {
        unreachable!();
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder> Decodable<D> for ! {
    fn decode(_d: &mut D) -> ! {
        unreachable!()
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder> Encodable<S> for NonZero<u32> {
    fn encode(&self, s: &mut S) {
        s.emit_u32(self.get());
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder> Decodable<D> for NonZero<u32> {
    fn decode(d: &mut D) -> Self {
        NonZero::new(d.read_u32()).unwrap()
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder> Encodable<S> for str {
    fn encode(&self, s: &mut S) {
        s.emit_str(self);
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder> Encodable<S> for String {
    fn encode(&self, s: &mut S) {
        s.emit_str(&self);
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder> Decodable<D> for String {
    fn decode(d: &mut D) -> String {
        d.read_str().to_owned()
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=4

```rust
impl<S: Encoder> Encodable<S> for () {
    fn encode(&self, _s: &mut S) {}
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=4

```rust
impl<D: Decoder> Decodable<D> for () {
    fn decode(_: &mut D) {}
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=4

```rust
impl<S: Encoder, T> Encodable<S> for PhantomData<T> {
    fn encode(&self, _s: &mut S) {}
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder, T> Decodable<D> for PhantomData<T> {
    fn decode(_: &mut D) -> PhantomData<T> {
        PhantomData
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Box<[T]> {
    fn decode(d: &mut D) -> Box<[T]> {
        let v: Vec<T> = Decodable::decode(d);
        v.into_boxed_slice()
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for Rc<T> {
    fn encode(&self, s: &mut S) {
        (**self).encode(s);
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Rc<T> {
    fn decode(d: &mut D) -> Rc<T> {
        Rc::new(Decodable::decode(d))
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=8 | LINES=9

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for [T] {
    default fn encode(&self, s: &mut S) {
        s.emit_usize(self.len());
        for e in self {
            e.encode(s);
        }
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for Vec<T> {
    fn encode(&self, s: &mut S) {
        self.as_slice().encode(s);
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Vec<T> {
    default fn decode(d: &mut D) -> Vec<T> {
        let len = d.read_usize();
        (0..len).map(|_| Decodable::decode(d)).collect()
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: Encodable<S>, const N: usize> Encodable<S> for [T; N] {
    fn encode(&self, s: &mut S) {
        self.as_slice().encode(s);
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=8 | LINES=12

```rust
impl<D: Decoder, const N: usize> Decodable<D> for [u8; N] {
    fn decode(d: &mut D) -> [u8; N] {
        let len = d.read_usize();
        assert!(len == N);
        let mut v = [0u8; N];
        for i in 0..len {
            v[i] = Decodable::decode(d);
        }
        v
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=10

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for Cow<'_, [T]>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn encode(&self, s: &mut S) {
        let slice: &[T] = self;
        slice.encode(s);
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=10

```rust
impl<D: Decoder, T: Decodable<D> + ToOwned> Decodable<D> for Cow<'static, [T]>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn decode(d: &mut D) -> Cow<'static, [T]> {
        let v: Vec<T> = Decodable::decode(d);
        Cow::Owned(v)
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=7

```rust
impl<S: Encoder> Encodable<S> for Cow<'_, str> {
    fn encode(&self, s: &mut S) {
        let val: &str = self;
        val.encode(s)
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder> Decodable<D> for Cow<'_, str> {
    fn decode(d: &mut D) -> Cow<'static, str> {
        let v: String = Decodable::decode(d);
        Cow::Owned(v)
    }
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=10 | LINES=12

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for Option<T> {
    fn encode(&self, s: &mut S) {
        match *self {
            None => s.emit_u8(0),
            Some(ref v) => {
                s.emit_u8(1);
                v.encode(s);
            }
        }
    }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=11 | LINES=10

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Option<T> {
    fn decode(d: &mut D) -> Option<T> {
        match d.read_u8() {
            0 => None,
            1 => Some(Decodable::decode(d)),
            _ => panic!("Encountered invalid discriminant while decoding `Option`."),
        }
    }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=11 | LINES=15

```rust
impl<S: Encoder, T1: Encodable<S>, T2: Encodable<S>> Encodable<S> for Result<T1, T2> {
    fn encode(&self, s: &mut S) {
        match *self {
            Ok(ref v) => {
                s.emit_u8(0);
                v.encode(s);
            }
            Err(ref v) => {
                s.emit_u8(1);
                v.encode(s);
            }
        }
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=11 | LINES=10

```rust
impl<D: Decoder, T1: Decodable<D>, T2: Decodable<D>> Decodable<D> for Result<T1, T2> {
    fn decode(d: &mut D) -> Result<T1, T2> {
        match d.read_u8() {
            0 => Ok(T1::decode(d)),
            1 => Err(T2::decode(d)),
            _ => panic!("Encountered invalid discriminant while decoding `Result`."),
        }
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=4

```rust
macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=18 | LINES=19

```rust
macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        impl<D: Decoder, $($name: Decodable<D>),+> Decodable<D> for ($($name,)+) {
            fn decode(d: &mut D) -> ($($name,)+) {
                ($({ let element: $name = Decodable::decode(d); element },)+)
            }
        }
        impl<S: Encoder, $($name: Encodable<S>),+> Encodable<S> for ($($name,)+) {
            #[allow(non_snake_case)]
            fn encode(&self, s: &mut S) {
                let ($(ref $name,)+) = *self;
                $($name.encode(s);)+
            }
        }
        peel! { $($name,)+ }
    )
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder> Encodable<S> for path::Path {
    fn encode(&self, e: &mut S) {
        self.to_str().unwrap().encode(e);
    }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder> Encodable<S> for path::PathBuf {
    fn encode(&self, e: &mut S) {
        path::Path::encode(self, e);
    }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder> Decodable<D> for path::PathBuf {
    fn decode(d: &mut D) -> path::PathBuf {
        let bytes: String = Decodable::decode(d);
        path::PathBuf::from(bytes)
    }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: Encodable<S> + Copy> Encodable<S> for Cell<T> {
    fn encode(&self, s: &mut S) {
        self.get().encode(s);
    }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder, T: Decodable<D> + Copy> Decodable<D> for Cell<T> {
    fn decode(d: &mut D) -> Cell<T> {
        Cell::new(Decodable::decode(d))
    }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for RefCell<T> {
    fn encode(&self, s: &mut S) {
        self.borrow().encode(s);
    }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for RefCell<T> {
    fn decode(d: &mut D) -> RefCell<T> {
        RefCell::new(Decodable::decode(d))
    }
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for Arc<T> {
    fn encode(&self, s: &mut S) {
        (**self).encode(s);
    }
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Arc<T> {
    fn decode(d: &mut D) -> Arc<T> {
        Arc::new(Decodable::decode(d))
    }
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: ?Sized + Encodable<S>> Encodable<S> for Box<T> {
    fn encode(&self, s: &mut S) {
        (**self).encode(s)
    }
}
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Box<T> {
    fn decode(d: &mut D) -> Box<T> {
        Box::new(Decodable::decode(d))
    }
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, A: Array<Item: Encodable<S>>> Encodable<S> for SmallVec<A> {
    fn encode(&self, s: &mut S) {
        self.as_slice().encode(s);
    }
}
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder, A: Array<Item: Decodable<D>>> Decodable<D> for SmallVec<A> {
    fn decode(d: &mut D) -> SmallVec<A> {
        let len = d.read_usize();
        (0..len).map(|_| Decodable::decode(d)).collect()
    }
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for ThinVec<T> {
    fn encode(&self, s: &mut S) {
        self.as_slice().encode(s);
    }
}
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for ThinVec<T> {
    fn decode(d: &mut D) -> ThinVec<T> {
        let len = d.read_usize();
        (0..len).map(|_| Decodable::decode(d)).collect()
    }
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=9

```rust
impl<S: Encoder, T: Encodable<S>> Encodable<S> for VecDeque<T> {
    fn encode(&self, s: &mut S) {
        s.emit_usize(self.len());
        for e in self {
            e.encode(s);
        }
    }
}
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for VecDeque<T> {
    fn decode(d: &mut D) -> VecDeque<T> {
        let len = d.read_usize();
        (0..len).map(|_| Decodable::decode(d)).collect()
    }
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=14

```rust
impl<S: Encoder, K, V> Encodable<S> for BTreeMap<K, V>
where
    K: Encodable<S> + PartialEq + Ord,
    V: Encodable<S>,
{
    fn encode(&self, e: &mut S) {
        e.emit_usize(self.len());
        for (key, val) in self {
            key.encode(e);
            val.encode(e);
        }
    }
}
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=11

```rust
impl<D: Decoder, K, V> Decodable<D> for BTreeMap<K, V>
where
    K: Decodable<D> + PartialEq + Ord,
    V: Decodable<D>,
{
    fn decode(d: &mut D) -> BTreeMap<K, V> {
        let len = d.read_usize();
        (0..len).map(|_| (Decodable::decode(d), Decodable::decode(d))).collect()
    }
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=12

```rust
impl<S: Encoder, T> Encodable<S> for BTreeSet<T>
where
    T: Encodable<S> + PartialEq + Ord,
{
    fn encode(&self, s: &mut S) {
        s.emit_usize(self.len());
        for e in self {
            e.encode(s);
        }
    }
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=10

```rust
impl<D: Decoder, T> Decodable<D> for BTreeSet<T>
where
    T: Decodable<D> + PartialEq + Ord,
{
    fn decode(d: &mut D) -> BTreeSet<T> {
        let len = d.read_usize();
        (0..len).map(|_| Decodable::decode(d)).collect()
    }
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=15

```rust
impl<E: Encoder, K, V, S> Encodable<E> for HashMap<K, V, S>
where
    K: Encodable<E> + Eq,
    V: Encodable<E>,
    S: BuildHasher,
{
    fn encode(&self, e: &mut E) {
        e.emit_usize(self.len());
        for (key, val) in self {
            key.encode(e);
            val.encode(e);
        }
    }
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=12

```rust
impl<D: Decoder, K, V, S> Decodable<D> for HashMap<K, V, S>
where
    K: Decodable<D> + Hash + Eq,
    V: Decodable<D>,
    S: BuildHasher + Default,
{
    fn decode(d: &mut D) -> HashMap<K, V, S> {
        let len = d.read_usize();
        (0..len).map(|_| (Decodable::decode(d), Decodable::decode(d))).collect()
    }
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=13

```rust
impl<E: Encoder, T, S> Encodable<E> for HashSet<T, S>
where
    T: Encodable<E> + Eq,
    S: BuildHasher,
{
    fn encode(&self, s: &mut E) {
        s.emit_usize(self.len());
        for e in self {
            e.encode(s);
        }
    }
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=11

```rust
impl<D: Decoder, T, S> Decodable<D> for HashSet<T, S>
where
    T: Decodable<D> + Hash + Eq,
    S: BuildHasher + Default,
{
    fn decode(d: &mut D) -> HashSet<T, S> {
        let len = d.read_usize();
        (0..len).map(|_| Decodable::decode(d)).collect()
    }
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=15

```rust
impl<E: Encoder, K, V, S> Encodable<E> for indexmap::IndexMap<K, V, S>
where
    K: Encodable<E> + Hash + Eq,
    V: Encodable<E>,
    S: BuildHasher,
{
    fn encode(&self, e: &mut E) {
        e.emit_usize(self.len());
        for (key, val) in self {
            key.encode(e);
            val.encode(e);
        }
    }
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=12

```rust
impl<D: Decoder, K, V, S> Decodable<D> for indexmap::IndexMap<K, V, S>
where
    K: Decodable<D> + Hash + Eq,
    V: Decodable<D>,
    S: BuildHasher + Default,
{
    fn decode(d: &mut D) -> indexmap::IndexMap<K, V, S> {
        let len = d.read_usize();
        (0..len).map(|_| (Decodable::decode(d), Decodable::decode(d))).collect()
    }
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=13

```rust
impl<E: Encoder, T, S> Encodable<E> for indexmap::IndexSet<T, S>
where
    T: Encodable<E> + Hash + Eq,
    S: BuildHasher,
{
    fn encode(&self, s: &mut E) {
        s.emit_usize(self.len());
        for e in self {
            e.encode(s);
        }
    }
}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=11

```rust
impl<D: Decoder, T, S> Decodable<D> for indexmap::IndexSet<T, S>
where
    T: Decodable<D> + Hash + Eq,
    S: BuildHasher + Default,
{
    fn decode(d: &mut D) -> indexmap::IndexSet<T, S> {
        let len = d.read_usize();
        (0..len).map(|_| Decodable::decode(d)).collect()
    }
}
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=7

```rust
impl<E: Encoder, T: Encodable<E>> Encodable<E> for Rc<[T]> {
    fn encode(&self, s: &mut E) {
        let slice: &[T] = self;
        slice.encode(s);
    }
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Rc<[T]> {
    fn decode(d: &mut D) -> Rc<[T]> {
        let vec: Vec<T> = Decodable::decode(d);
        vec.into()
    }
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=7

```rust
impl<E: Encoder, T: Encodable<E>> Encodable<E> for Arc<[T]> {
    fn encode(&self, s: &mut E) {
        let slice: &[T] = self;
        slice.encode(s);
    }
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder, T: Decodable<D>> Decodable<D> for Arc<[T]> {
    fn decode(d: &mut D) -> Arc<[T]> {
        let vec: Vec<T> = Decodable::decode(d);
        vec.into()
    }
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=7

```rust
impl<S: Encoder> Encodable<S> for Hash64 {
    #[inline]
    fn encode(&self, s: &mut S) {
        s.emit_raw_bytes(&self.as_u64().to_le_bytes());
    }
}
```

## Block 78
**Metadata**: AST_ID=78 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=7

```rust
impl<S: Encoder> Encodable<S> for Hash128 {
    #[inline]
    fn encode(&self, s: &mut S) {
        s.emit_raw_bytes(&self.as_u128().to_le_bytes());
    }
}
```

## Block 79
**Metadata**: AST_ID=79 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder> Decodable<D> for Hash64 {
    #[inline]
    fn decode(d: &mut D) -> Self {
        Self::new(u64::from_le_bytes(d.read_raw_bytes(8).try_into().unwrap()))
    }
}
```

## Block 80
**Metadata**: AST_ID=80 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=7

```rust
impl<D: Decoder> Decodable<D> for Hash128 {
    #[inline]
    fn decode(d: &mut D) -> Self {
        Self::new(u128::from_le_bytes(d.read_raw_bytes(16).try_into().unwrap()))
    }
}
```

---
*Generated by AST tracing system*
