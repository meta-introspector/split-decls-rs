# AST Trace: ../rust/library/proc_macro/src/bridge/rpc.rs

Generated 30 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=4 | LINES=12

```rust
//! Serialization for client-server communication.

use std::any::Any;
use std::io::Write;
use std::num::NonZero;
use std::str;

pub(super) type Writer = super::buffer::Buffer;

pub(super) trait Encode<S>: Sized {
    fn encode(self, w: &mut Writer, s: &mut S);
}
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=2 | LINES=6

```rust
pub(super) type Reader<'a> = &'a [u8];

pub(super) trait Decode<'a, 's, S>: Sized {
    fn decode(r: &mut Reader<'a>, s: &'s S) -> Self;
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=2 | LINES=4

```rust
pub(super) trait DecodeMut<'a, 's, S>: Sized {
    fn decode(r: &mut Reader<'a>, s: &'s mut S) -> Self;
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=56 | LINES=83

```rust
macro_rules! rpc_encode_decode {
    (le $ty:ty) => {
        impl<S> Encode<S> for $ty {
            fn encode(self, w: &mut Writer, _: &mut S) {
                w.extend_from_array(&self.to_le_bytes());
            }
        }

        impl<S> DecodeMut<'_, '_, S> for $ty {
            fn decode(r: &mut Reader<'_>, _: &mut S) -> Self {
                const N: usize = ::std::mem::size_of::<$ty>();

                let mut bytes = [0; N];
                bytes.copy_from_slice(&r[..N]);
                *r = &r[N..];

                Self::from_le_bytes(bytes)
            }
        }
    };
    (struct $name:ident $(<$($T:ident),+>)? { $($field:ident),* $(,)? }) => {
        impl<S, $($($T: Encode<S>),+)?> Encode<S> for $name $(<$($T),+>)? {
            fn encode(self, w: &mut Writer, s: &mut S) {
                $(self.$field.encode(w, s);)*
            }
        }

        impl<'a, S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
            for $name $(<$($T),+>)?
        {
            fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
                $name {
                    $($field: DecodeMut::decode(r, s)),*
                }
            }
        }
    };
    (enum $name:ident $(<$($T:ident),+>)? { $($variant:ident $(($field:ident))*),* $(,)? }) => {
        impl<S, $($($T: Encode<S>),+)?> Encode<S> for $name $(<$($T),+>)? {
            fn encode(self, w: &mut Writer, s: &mut S) {
                // HACK(eddyb): `Tag` enum duplicated between the
                // two impls as there's no other place to stash it.
                #[allow(non_upper_case_globals)]
                mod tag {
                    #[repr(u8)] enum Tag { $($variant),* }

                    $(pub(crate) const $variant: u8 = Tag::$variant as u8;)*
                }

                match self {
                    $($name::$variant $(($field))* => {
                        tag::$variant.encode(w, s);
                        $($field.encode(w, s);)*
                    })*
                }
            }
        }

        impl<'a, S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
            for $name $(<$($T),+>)?
        {
            fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
                // HACK(eddyb): `Tag` enum duplicated between the
                // two impls as there's no other place to stash it.
                #[allow(non_upper_case_globals)]
                mod tag {
                    #[repr(u8)] enum Tag { $($variant),* }

                    $(pub(crate) const $variant: u8 = Tag::$variant as u8;)*
                }

                match u8::decode(r, s) {
                    $(tag::$variant => {
                        $(let $field = DecodeMut::decode(r, s);)*
                        $name::$variant $(($field))*
                    })*
                    _ => unreachable!(),
                }
            }
        }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=4

```rust
impl<S> Encode<S> for () {
    fn encode(self, _: &mut Writer, _: &mut S) {}
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=4

```rust
impl<S> DecodeMut<'_, '_, S> for () {
    fn decode(_: &mut Reader<'_>, _: &mut S) -> Self {}
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S> Encode<S> for u8 {
    fn encode(self, w: &mut Writer, _: &mut S) {
        w.push(self);
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=8

```rust
impl<S> DecodeMut<'_, '_, S> for u8 {
    fn decode(r: &mut Reader<'_>, _: &mut S) -> Self {
        let x = r[0];
        *r = &r[1..];
        x
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=9

```rust
rpc_encode_decode!(le u32);
rpc_encode_decode!(le usize);

impl<S> Encode<S> for bool {
    fn encode(self, w: &mut Writer, s: &mut S) {
        (self as u8).encode(w, s);
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=9 | LINES=10

```rust
impl<S> DecodeMut<'_, '_, S> for bool {
    fn decode(r: &mut Reader<'_>, s: &mut S) -> Self {
        match u8::decode(r, s) {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S> Encode<S> for char {
    fn encode(self, w: &mut Writer, s: &mut S) {
        (self as u32).encode(w, s);
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<S> DecodeMut<'_, '_, S> for char {
    fn decode(r: &mut Reader<'_>, s: &mut S) -> Self {
        char::from_u32(u32::decode(r, s)).unwrap()
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S> Encode<S> for NonZero<u32> {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self.get().encode(w, s);
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<S> DecodeMut<'_, '_, S> for NonZero<u32> {
    fn decode(r: &mut Reader<'_>, s: &mut S) -> Self {
        Self::new(u32::decode(r, s)).unwrap()
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=7

```rust
impl<S, A: Encode<S>, B: Encode<S>> Encode<S> for (A, B) {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self.0.encode(w, s);
        self.1.encode(w, s);
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=8

```rust
impl<'a, S, A: for<'s> DecodeMut<'a, 's, S>, B: for<'s> DecodeMut<'a, 's, S>> DecodeMut<'a, '_, S>
    for (A, B)
{
    fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
        (DecodeMut::decode(r, s), DecodeMut::decode(r, s))
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=7

```rust
impl<S> Encode<S> for &[u8] {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self.len().encode(w, s);
        w.write_all(self).unwrap();
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=9

```rust
impl<'a, S> DecodeMut<'a, '_, S> for &'a [u8] {
    fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
        let len = usize::decode(r, s);
        let xs = &r[..len];
        *r = &r[len..];
        xs
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S> Encode<S> for &str {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self.as_bytes().encode(w, s);
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<'a, S> DecodeMut<'a, '_, S> for &'a str {
    fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
        str::from_utf8(<&[u8]>::decode(r, s)).unwrap()
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S> Encode<S> for String {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self[..].encode(w, s);
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=5 | LINES=6

```rust
impl<S> DecodeMut<'_, '_, S> for String {
    fn decode(r: &mut Reader<'_>, s: &mut S) -> Self {
        <&str>::decode(r, s).to_string()
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=8 | LINES=9

```rust
impl<S, T: Encode<S>> Encode<S> for Vec<T> {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self.len().encode(w, s);
        for x in self {
            x.encode(w, s);
        }
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=8 | LINES=11

```rust
impl<'a, S, T: for<'s> DecodeMut<'a, 's, S>> DecodeMut<'a, '_, S> for Vec<T> {
    fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
        let len = usize::decode(r, s);
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::decode(r, s));
        }
        vec
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
/// Simplified version of panic payloads, ignoring
/// types other than `&'static str` and `String`.
pub enum PanicMessage {
    StaticStr(&'static str),
    String(String),
    Unknown,
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=from | COMPLEXITY=11 | LINES=12

```rust
impl From<Box<dyn Any + Send>> for PanicMessage {
    fn from(payload: Box<dyn Any + Send + 'static>) -> Self {
        if let Some(s) = payload.downcast_ref::<&'static str>() {
            return PanicMessage::StaticStr(s);
        }
        if let Ok(s) = payload.downcast::<String>() {
            return PanicMessage::String(*s);
        }
        PanicMessage::Unknown
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=from | COMPLEXITY=10 | LINES=13

```rust
impl From<PanicMessage> for Box<dyn Any + Send> {
    fn from(val: PanicMessage) -> Self {
        match val {
            PanicMessage::StaticStr(s) => Box::new(s),
            PanicMessage::String(s) => Box::new(s),
            PanicMessage::Unknown => {
                struct UnknownPanicMessage;
                Box::new(UnknownPanicMessage)
            }
        }
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=as_str | COMPLEXITY=7 | LINES=10

```rust
impl PanicMessage {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            PanicMessage::StaticStr(s) => Some(s),
            PanicMessage::String(s) => Some(s),
            PanicMessage::Unknown => None,
        }
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=5 | LINES=6

```rust
impl<S> Encode<S> for PanicMessage {
    fn encode(self, w: &mut Writer, s: &mut S) {
        self.as_str().encode(w, s);
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=9 | LINES=9

```rust
impl<S> DecodeMut<'_, '_, S> for PanicMessage {
    fn decode(r: &mut Reader<'_>, s: &mut S) -> Self {
        match Option::<String>::decode(r, s) {
            Some(s) => PanicMessage::String(s),
            None => PanicMessage::Unknown,
        }
    }
}
```

---
*Generated by AST tracing system*
