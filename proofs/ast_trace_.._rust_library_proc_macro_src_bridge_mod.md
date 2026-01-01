# AST Trace: ../rust/library/proc_macro/src/bridge/mod.rs

Generated 52 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=5 | LINES=12

```rust
//! Internal interface for communicating between a `proc_macro` client
//! (a proc macro crate) and a `proc_macro` server (a compiler front-end).
//!
//! Serialization (with C ABI buffers) and unique integer handles are employed
//! to allow safely interfacing between two copies of `proc_macro` built
//! (from the same source) by different compilers with potentially mismatching
//! Rust ABIs (e.g., stage0/bin/rustc vs stage1/bin/rustc during bootstrap).

#![deny(unsafe_code)]

use std::hash::Hash;
use std::ops::{Bound, Range};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use std::sync::Once;
use std::{fmt, marker, mem, panic, thread};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::{Delimiter, Level, Spacing};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=17

```rust
/// Higher-order macro describing the server RPC API, allowing automatic
/// generation of type-safe Rust APIs, both client-side and server-side.
///
/// `with_api!(MySelf, my_self, my_macro)` expands to:
/// ```rust,ignore (pseudo-code)
/// my_macro! {
///     // ...
///     Literal {
///         // ...
///         fn character(ch: char) -> MySelf::Literal;
///         // ...
///         fn span(my_self: &MySelf::Literal) -> MySelf::Span;
///         fn set_span(my_self: &mut MySelf::Literal, span: MySelf::Span);
///     },
///     // ...
/// }
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=24 | LINES=71

```rust
/// ```
///
/// The first two arguments serve to customize the arguments names
/// and argument/return types, to enable several different usecases:
///
/// If `my_self` is just `self`, then each `fn` signature can be used
/// as-is for a method. If it's anything else (`self_` in practice),
/// then the signatures don't have a special `self` argument, and
/// can, therefore, have a different one introduced.
///
/// If `MySelf` is just `Self`, then the types are only valid inside
/// a trait or a trait impl, where the trait has associated types
/// for each of the API types. If non-associated types are desired,
/// a module name (`self` in practice) can be used instead of `Self`.
macro_rules! with_api {
    ($S:ident, $self:ident, $m:ident) => {
        $m! {
            FreeFunctions {
                fn drop($self: $S::FreeFunctions);
                fn injected_env_var(var: &str) -> Option<String>;
                fn track_env_var(var: &str, value: Option<&str>);
                fn track_path(path: &str);
                fn literal_from_str(s: &str) -> Result<Literal<$S::Span, $S::Symbol>, ()>;
                fn emit_diagnostic(diagnostic: Diagnostic<$S::Span>);
            },
            TokenStream {
                fn drop($self: $S::TokenStream);
                fn clone($self: &$S::TokenStream) -> $S::TokenStream;
                fn is_empty($self: &$S::TokenStream) -> bool;
                fn expand_expr($self: &$S::TokenStream) -> Result<$S::TokenStream, ()>;
                fn from_str(src: &str) -> $S::TokenStream;
                fn to_string($self: &$S::TokenStream) -> String;
                fn from_token_tree(
                    tree: TokenTree<$S::TokenStream, $S::Span, $S::Symbol>,
                ) -> $S::TokenStream;
                fn concat_trees(
                    base: Option<$S::TokenStream>,
                    trees: Vec<TokenTree<$S::TokenStream, $S::Span, $S::Symbol>>,
                ) -> $S::TokenStream;
                fn concat_streams(
                    base: Option<$S::TokenStream>,
                    streams: Vec<$S::TokenStream>,
                ) -> $S::TokenStream;
                fn into_trees(
                    $self: $S::TokenStream
                ) -> Vec<TokenTree<$S::TokenStream, $S::Span, $S::Symbol>>;
            },
            Span {
                fn debug($self: $S::Span) -> String;
                fn parent($self: $S::Span) -> Option<$S::Span>;
                fn source($self: $S::Span) -> $S::Span;
                fn byte_range($self: $S::Span) -> Range<usize>;
                fn start($self: $S::Span) -> $S::Span;
                fn end($self: $S::Span) -> $S::Span;
                fn line($self: $S::Span) -> usize;
                fn column($self: $S::Span) -> usize;
                fn file($self: $S::Span) -> String;
                fn local_file($self: $S::Span) -> Option<String>;
                fn join($self: $S::Span, other: $S::Span) -> Option<$S::Span>;
                fn subspan($self: $S::Span, start: Bound<usize>, end: Bound<usize>) -> Option<$S::Span>;
                fn resolved_at($self: $S::Span, at: $S::Span) -> $S::Span;
                fn source_text($self: $S::Span) -> Option<String>;
                fn save_span($self: $S::Span) -> usize;
                fn recover_proc_macro_span(id: usize) -> $S::Span;
            },
            Symbol {
                fn normalize_and_validate_ident(string: &str) -> Result<$S::Symbol, ()>;
            },
        }
    };
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=16

```rust
// Similar to `with_api`, but only lists the types requiring handles, and they
// are divided into the two storage categories.
macro_rules! with_api_handle_types {
    ($m:ident) => {
        $m! {
            'owned:
            FreeFunctions,
            TokenStream,

            'interned:
            Span,
            // Symbol is handled manually
        }
    };
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=14 | LINES=10

```rust
// FIXME(eddyb) this calls `encode` for each argument, but in reverse,
// to match the ordering in `reverse_decode`.
macro_rules! reverse_encode {
    ($writer:ident;) => {};
    ($writer:ident; $first:ident $(, $rest:ident)*) => {
        reverse_encode!($writer; $($rest),*);
        $first.encode(&mut $writer, &mut ());
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=11 | LINES=10

```rust
// FIXME(eddyb) this calls `decode` for each argument, but in reverse,
// to avoid borrow conflicts from borrows started by `&mut` arguments.
macro_rules! reverse_decode {
    ($reader:ident, $s:ident;) => {};
    ($reader:ident, $s:ident; $first:ident: $first_ty:ty $(, $rest:ident: $rest_ty:ty)*) => {
        reverse_decode!($reader, $s; $($rest: $rest_ty),*);
        let $first = <$first_ty>::decode(&mut $reader, $s);
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=26

```rust
#[allow(unsafe_code)]
mod arena;
#[allow(unsafe_code)]
mod buffer;
#[deny(unsafe_code)]
pub mod client;
#[allow(unsafe_code)]
mod closure;
#[forbid(unsafe_code)]
mod fxhash;
#[forbid(unsafe_code)]
mod handle;
#[macro_use]
#[forbid(unsafe_code)]
mod rpc;
#[allow(unsafe_code)]
mod selfless_reify;
#[forbid(unsafe_code)]
pub mod server;
#[allow(unsafe_code)]
mod symbol;

use buffer::Buffer;
pub use rpc::PanicMessage;
use rpc::{Decode, DecodeMut, Encode, Reader, Writer};
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=6

```rust
/// Configuration for establishing an active connection between a server and a
/// client.  The server creates the bridge config (`run_server` in `server.rs`),
/// then passes it to the client through the function pointer in the `run` field
/// of `client::Client`. The client constructs a local `Bridge` from the config
/// in TLS during its execution (`Bridge::{enter, with}` in `client.rs`).
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=BridgeConfig | COMPLEXITY=3 | LINES=16

```rust
#[repr(C)]
pub struct BridgeConfig<'a> {
    /// Buffer used to pass initial input to the client.
    input: Buffer,

    /// Server-side function that the client uses to make requests.
    dispatch: closure::Closure<'a, Buffer, Buffer>,

    /// If 'true', always invoke the default panic hook
    force_show_panics: bool,

    // Prevent Send and Sync impls. `!Send`/`!Sync` is the usual way of doing
    // this, but that requires unstable features. rust-analyzer uses this code
    // and avoids unstable features.
    _marker: marker::PhantomData<*mut ()>,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=16 | LINES=25

```rust
#[forbid(unsafe_code)]
#[allow(non_camel_case_types)]
mod api_tags {
    use super::rpc::{DecodeMut, Encode, Reader, Writer};

    macro_rules! declare_tags {
        ($($name:ident {
            $(fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?) $(-> $ret_ty:ty)*;)*
        }),* $(,)?) => {
            $(
                pub(super) enum $name {
                    $($method),*
                }
                rpc_encode_decode!(enum $name { $($method),* });
            )*

            pub(super) enum Method {
                $($name($name)),*
            }
            rpc_encode_decode!(enum Method { $($name(m)),* });
        }
    }
    with_api!(self, self, declare_tags);
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=mark | COMPLEXITY=6 | LINES=9

```rust
/// Helper to wrap associated types to allow trait impl dispatch.
/// That is, normally a pair of impls for `T::Foo` and `T::Bar`
/// can overlap, but if the impls are, instead, on types like
/// `Marked<T::Foo, Foo>` and `Marked<T::Bar, Bar>`, they can't.
trait Mark {
    type Unmarked;
    fn mark(unmarked: Self::Unmarked) -> Self;
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=unmark | COMPLEXITY=4 | LINES=6

```rust
/// Unwrap types wrapped by `Mark::mark` (see `Mark` for details).
trait Unmark {
    type Unmarked;
    fn unmark(self) -> Self::Unmarked;
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=STRUCT | NAME=Marked | COMPLEXITY=2 | LINES=6

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Marked<T, M> {
    value: T,
    _marker: marker::PhantomData<M>,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=mark | COMPLEXITY=6 | LINES=7

```rust
impl<T, M> Mark for Marked<T, M> {
    type Unmarked = T;
    fn mark(unmarked: Self::Unmarked) -> Self {
        Marked { value: unmarked, _marker: marker::PhantomData }
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=unmark | COMPLEXITY=5 | LINES=6

```rust
impl<T, M> Unmark for Marked<T, M> {
    type Unmarked = T;
    fn unmark(self) -> Self::Unmarked {
        self.value
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=unmark | COMPLEXITY=5 | LINES=6

```rust
impl<'a, T, M> Unmark for &'a Marked<T, M> {
    type Unmarked = &'a T;
    fn unmark(self) -> Self::Unmarked {
        &self.value
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=unmark | COMPLEXITY=5 | LINES=6

```rust
impl<'a, T, M> Unmark for &'a mut Marked<T, M> {
    type Unmarked = &'a mut T;
    fn unmark(self) -> Self::Unmarked {
        &mut self.value
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=mark | COMPLEXITY=5 | LINES=8

```rust
impl<T: Mark> Mark for Vec<T> {
    type Unmarked = Vec<T::Unmarked>;
    fn mark(unmarked: Self::Unmarked) -> Self {
        // Should be a no-op due to std's in-place collect optimizations.
        unmarked.into_iter().map(T::mark).collect()
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=unmark | COMPLEXITY=5 | LINES=7

```rust
impl<T: Unmark> Unmark for Vec<T> {
    type Unmarked = Vec<T::Unmarked>;
    fn unmark(self) -> Self::Unmarked {
        // Should be a no-op due to std's in-place collect optimizations.
        self.into_iter().map(T::unmark).collect()
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=mark | COMPLEXITY=16 | LINES=19

```rust
macro_rules! mark_noop {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Mark for $ty {
                type Unmarked = Self;
                fn mark(unmarked: Self::Unmarked) -> Self {
                    unmarked
                }
            }
            impl Unmark for $ty {
                type Unmarked = Self;
                fn unmark(self) -> Self::Unmarked {
                    self
                }
            }
        )*
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
mark_noop! {
    (),
    bool,
    char,
    &'_ [u8],
    &'_ str,
    String,
    u8,
    usize,
    Delimiter,
    LitKind,
    Level,
    Spacing,
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
rpc_encode_decode!(
    enum Delimiter {
        Parenthesis,
        Brace,
        Bracket,
        None,
    }
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
);
rpc_encode_decode!(
    enum Level {
        Error,
        Warning,
        Note,
        Help,
    }
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
);
rpc_encode_decode!(
    enum Spacing {
        Alone,
        Joint,
    }
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=20

```rust
);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum LitKind {
    Byte,
    Char,
    Integer,
    Float,
    Str,
    StrRaw(u8),
    ByteStr,
    ByteStrRaw(u8),
    CStr,
    CStrRaw(u8),
    // This should have an `ErrorGuaranteed`, except that type isn't available
    // in this crate. (Imagine it is there.) Hence the `WithGuar` suffix. Must
    // only be constructed in `LitKind::from_internal`, where an
    // `ErrorGuaranteed` is available.
    ErrWithGuar,
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
rpc_encode_decode!(
    enum LitKind {
        Byte,
        Char,
        Integer,
        Float,
        Str,
        StrRaw(n),
        ByteStr,
        ByteStrRaw(n),
        CStr,
        CStrRaw(n),
        ErrWithGuar,
    }
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=mark | COMPLEXITY=42 | LINES=46

```rust
);

macro_rules! mark_compound {
    (struct $name:ident <$($T:ident),+> { $($field:ident),* $(,)? }) => {
        impl<$($T: Mark),+> Mark for $name <$($T),+> {
            type Unmarked = $name <$($T::Unmarked),+>;
            fn mark(unmarked: Self::Unmarked) -> Self {
                $name {
                    $($field: Mark::mark(unmarked.$field)),*
                }
            }
        }

        impl<$($T: Unmark),+> Unmark for $name <$($T),+> {
            type Unmarked = $name <$($T::Unmarked),+>;
            fn unmark(self) -> Self::Unmarked {
                $name {
                    $($field: Unmark::unmark(self.$field)),*
                }
            }
        }
    };
    (enum $name:ident <$($T:ident),+> { $($variant:ident $(($field:ident))?),* $(,)? }) => {
        impl<$($T: Mark),+> Mark for $name <$($T),+> {
            type Unmarked = $name <$($T::Unmarked),+>;
            fn mark(unmarked: Self::Unmarked) -> Self {
                match unmarked {
                    $($name::$variant $(($field))? => {
                        $name::$variant $((Mark::mark($field)))?
                    })*
                }
            }
        }

        impl<$($T: Unmark),+> Unmark for $name <$($T),+> {
            type Unmarked = $name <$($T::Unmarked),+>;
            fn unmark(self) -> Self::Unmarked {
                match self {
                    $($name::$variant $(($field))? => {
                        $name::$variant $((Unmark::unmark($field)))?
                    })*
                }
            }
        }
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=7

```rust
macro_rules! compound_traits {
    ($($t:tt)*) => {
        rpc_encode_decode!($($t)*);
        mark_compound!($($t)*);
    };
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
compound_traits!(
    enum Bound<T> {
        Included(x),
        Excluded(x),
        Unbounded,
    }
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
);

compound_traits!(
    enum Option<T> {
        Some(t),
        None,
    }
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
);

compound_traits!(
    enum Result<T, E> {
        Ok(t),
        Err(e),
    }
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=DelimSpan | COMPLEXITY=2 | LINES=8

```rust
);

#[derive(Copy, Clone)]
pub struct DelimSpan<Span> {
    pub open: Span,
    pub close: Span,
    pub entire: Span,
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=from_single | COMPLEXITY=4 | LINES=6

```rust
impl<Span: Copy> DelimSpan<Span> {
    pub fn from_single(span: Span) -> Self {
        DelimSpan { open: span, close: span, entire: span }
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
compound_traits!(struct DelimSpan<Span> { open, close, entire });
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=STRUCT | NAME=Group | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone)]
pub struct Group<TokenStream, Span> {
    pub delimiter: Delimiter,
    pub stream: Option<TokenStream>,
    pub span: DelimSpan<Span>,
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
compound_traits!(struct Group<TokenStream, Span> { delimiter, stream, span });
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=STRUCT | NAME=Punct | COMPLEXITY=2 | LINES=7

```rust
#[derive(Clone)]
pub struct Punct<Span> {
    pub ch: u8,
    pub joint: bool,
    pub span: Span,
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
compound_traits!(struct Punct<Span> { ch, joint, span });
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=STRUCT | NAME=Ident | COMPLEXITY=2 | LINES=7

```rust
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ident<Span, Symbol> {
    pub sym: Symbol,
    pub is_raw: bool,
    pub span: Span,
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
compound_traits!(struct Ident<Span, Symbol> { sym, is_raw, span });
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=STRUCT | NAME=Literal | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone, Eq, PartialEq)]
pub struct Literal<Span, Symbol> {
    pub kind: LitKind,
    pub symbol: Symbol,
    pub suffix: Option<Symbol>,
    pub span: Span,
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
compound_traits!(struct Literal<Sp, Sy> { kind, symbol, suffix, span });
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Clone)]
pub enum TokenTree<TokenStream, Span, Symbol> {
    Group(Group<TokenStream, Span>),
    Punct(Punct<Span>),
    Ident(Ident<Span, Symbol>),
    Literal(Literal<Span, Symbol>),
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
compound_traits!(
    enum TokenTree<TokenStream, Span, Symbol> {
        Group(tt),
        Punct(tt),
        Ident(tt),
        Literal(tt),
    }
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=STRUCT | NAME=Diagnostic | COMPLEXITY=2 | LINES=9

```rust
);

#[derive(Clone, Debug)]
pub struct Diagnostic<Span> {
    pub level: Level,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Diagnostic<Span>>,
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=STRUCT | NAME=Diagnostic | COMPLEXITY=2 | LINES=3

```rust
compound_traits!(
    struct Diagnostic<Span> { level, message, spans, children }
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=STRUCT | NAME=ExpnGlobals | COMPLEXITY=4 | LINES=10

```rust
);

/// Globals provided alongside the initial inputs for a macro expansion.
/// Provides values such as spans which are used frequently to avoid RPC.
#[derive(Clone)]
pub struct ExpnGlobals<Span> {
    pub def_site: Span,
    pub call_site: Span,
    pub mixed_site: Span,
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=STRUCT | NAME=ExpnGlobals | COMPLEXITY=2 | LINES=3

```rust
compound_traits!(
    struct ExpnGlobals<Span> { def_site, call_site, mixed_site }
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=STRUCT | NAME=Range | COMPLEXITY=2 | LINES=4

```rust
);

compound_traits!(
    struct Range<T> { start, end }
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=1

```rust
);
```

---
*Generated by AST tracing system*
