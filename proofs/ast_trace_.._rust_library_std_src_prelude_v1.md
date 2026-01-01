# AST Trace: ../rust/library/std/src/prelude/v1.rs

Generated 13 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=13

```rust
//! The first version of the prelude of The Rust Standard Library.
//!
//! See the [module-level documentation](super) for more.

#![stable(feature = "rust1", since = "1.0.0")]

// No formatting: this file is nothing but re-exports, and their order is worth preserving.
#![cfg_attr(rustfmt, rustfmt::skip)]

// Re-exported core operators
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::marker::{Send, Sized, Sync, Unpin};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::ops::{Drop, Fn, FnMut, FnOnce};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[stable(feature = "async_closure", since = "1.85.0")]
#[doc(no_inline)]
pub use crate::ops::{AsyncFn, AsyncFnMut, AsyncFnOnce};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
// Re-exported functions
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::mem::drop;
#[stable(feature = "size_of_prelude", since = "1.80.0")]
#[doc(no_inline)]
pub use crate::mem::{align_of, align_of_val, size_of, size_of_val};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
// Re-exported types and traits
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::convert::{AsMut, AsRef, From, Into};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::iter::{DoubleEndedIterator, ExactSizeIterator};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::iter::{Extend, IntoIterator, Iterator};
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::option::Option::{self, None, Some};
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::result::Result::{self, Err, Ok};
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
// Re-exported built-in macros
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(no_inline)]
pub use core::prelude::v1::{
    assert, cfg, column, compile_error, concat, env, file, format_args,
    format_args_nl, include, include_bytes, include_str, line, log_syntax, module_path, option_env,
    stringify, trace_macros, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
};
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=7 | LINES=15

```rust
#[unstable(
    feature = "concat_bytes",
    issue = "87555",
    reason = "`concat_bytes` is not stable enough for use and is subject to change"
)]
#[doc(no_inline)]
pub use core::prelude::v1::concat_bytes;

// Do not `doc(no_inline)` so that they become doc items on their own
// (no public module for them to be re-exported from).
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
pub use core::prelude::v1::{
    alloc_error_handler, bench, derive, global_allocator, test, test_case,
};
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=9 | LINES=58

```rust
#[unstable(feature = "derive_const", issue = "118304")]
pub use core::prelude::v1::derive_const;

// Do not `doc(no_inline)` either.
#[unstable(
    feature = "cfg_accessible",
    issue = "64797",
    reason = "`cfg_accessible` is not fully implemented"
)]
pub use core::prelude::v1::cfg_accessible;

// Do not `doc(no_inline)` either.
#[unstable(
    feature = "cfg_eval",
    issue = "82679",
    reason = "`cfg_eval` is a recently implemented feature"
)]
pub use core::prelude::v1::cfg_eval;

// Do not `doc(no_inline)` either.
#[unstable(
    feature = "type_ascription",
    issue = "23416",
    reason = "placeholder syntax for type ascription"
)]
pub use core::prelude::v1::type_ascribe;

// Do not `doc(no_inline)` either.
#[unstable(
    feature = "deref_patterns",
    issue = "87121",
    reason = "placeholder syntax for deref patterns"
)]
pub use core::prelude::v1::deref;

// Do not `doc(no_inline)` either.
#[unstable(
    feature = "type_alias_impl_trait",
    issue = "63063",
    reason = "`type_alias_impl_trait` has open design concerns"
)]
pub use core::prelude::v1::define_opaque;

// The file so far is equivalent to core/src/prelude/v1.rs. It is duplicated
// rather than glob imported because we want docs to show these re-exports as
// pointing to within `std`.
// Below are the items from the alloc crate.

#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::borrow::ToOwned;
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::boxed::Box;
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::string::{String, ToString};
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=1 | LINES=3

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[doc(no_inline)]
pub use crate::vec::Vec;
```

---
*Generated by AST tracing system*
