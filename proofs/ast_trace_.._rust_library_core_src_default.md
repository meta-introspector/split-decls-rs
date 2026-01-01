# AST Trace: ../rust/library/core/src/default.rs

Generated 32 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=18

```rust
//! The `Default` trait for types with a default value.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::ascii::Char as AsciiChar;

/// A trait for giving a type a useful default value.
///
/// Sometimes, you want to fall back to some kind of default value, and
/// don't particularly care what it is. This comes up often with `struct`s
/// that define a set of options:
///
/// ```
/// # #[allow(dead_code)]
/// struct SomeOptions {
///     foo: i32,
///     bar: f32,
/// }
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
/// ```
///
/// How can we define some default values? You can use `Default`:
///
/// ```
/// # #[allow(dead_code)]
/// #[derive(Default)]
/// struct SomeOptions {
///     foo: i32,
///     bar: f32,
/// }
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
///
/// fn main() {
///     let options: SomeOptions = Default::default();
/// }
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=13

```rust
/// ```
///
/// Now, you get all of the default values. Rust implements `Default` for various primitive types.
///
/// If you want to override a particular option, but still retain the other defaults:
///
/// ```
/// # #[allow(dead_code)]
/// # #[derive(Default)]
/// # struct SomeOptions {
/// #     foo: i32,
/// #     bar: f32,
/// # }
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=3

```rust
/// fn main() {
///     let options = SomeOptions { foo: 42, ..Default::default() };
/// }
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=20

```rust
/// ```
///
/// ## Derivable
///
/// This trait can be used with `#[derive]` if all of the type's fields implement
/// `Default`. When `derive`d, it will use the default value for each field's type.
///
/// ### `enum`s
///
/// When using `#[derive(Default)]` on an `enum`, you need to choose which unit variant will be
/// default. You do this by placing the `#[default]` attribute on the variant.
///
/// ```
/// #[derive(Default)]
/// enum Kind {
///     #[default]
///     A,
///     B,
///     C,
/// }
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=18

```rust
/// ```
///
/// You cannot use the `#[default]` attribute on non-unit or non-exhaustive variants.
///
/// The `#[default]` attribute was stabilized in Rust 1.62.0.
///
/// ## How can I implement `Default`?
///
/// Provide an implementation for the `default()` method that returns the value of
/// your type that should be the default:
///
/// ```
/// # #![allow(dead_code)]
/// enum Kind {
///     A,
///     B,
///     C,
/// }
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=4

```rust
///
/// impl Default for Kind {
///     fn default() -> Self { Kind::A }
/// }
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
/// ```
///
/// # Examples
///
/// ```
/// # #[allow(dead_code)]
/// #[derive(Default)]
/// struct SomeOptions {
///     foo: i32,
///     bar: f32,
/// }
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=default | COMPLEXITY=11 | LINES=38

```rust
/// ```
#[rustc_diagnostic_item = "Default"]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "const_default", issue = "143894")]
pub const trait Default: Sized {
    /// Returns the "default value" for a type.
    ///
    /// Default values are often some kind of initial value, identity value, or anything else that
    /// may make sense as a default.
    ///
    /// # Examples
    ///
    /// Using built-in default values:
    ///
    /// ```
    /// let i: i8 = Default::default();
    /// let (x, y): (Option<String>, f64) = Default::default();
    /// let (a, b, (c, d)): (i32, u32, (bool, bool)) = Default::default();
    /// ```
    ///
    /// Making your own:
    ///
    /// ```
    /// # #[allow(dead_code)]
    /// enum Kind {
    ///     A,
    ///     B,
    ///     C,
    /// }
    ///
    /// impl Default for Kind {
    ///     fn default() -> Self { Kind::A }
    /// }
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "default_fn"]
    fn default() -> Self;
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
/// Derive macro generating an impl of the trait `Default`.
#[rustc_builtin_macro(Default, attributes(default))]
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[allow_internal_unstable(core_intrinsics)]
pub macro Default($item:item) {
    /* compiler built-in */
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=default | COMPLEXITY=12 | LINES=14

```rust
macro_rules! default_impl {
    ($t:ty, $v:expr, $doc:tt) => {
        #[stable(feature = "rust1", since = "1.0.0")]
        #[rustc_const_unstable(feature = "const_default", issue = "143894")]
        impl const Default for $t {
            #[inline(always)]
            #[doc = $doc]
            fn default() -> $t {
                $v
            }
        }
    };
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
default_impl! { (), (), "Returns the default value of `()`" }
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { bool, false, "Returns the default value of `false`" }
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { char, '\x00', "Returns the default value of `\\x00`" }
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { AsciiChar, AsciiChar::Null, "Returns the default value of `Null`" }
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
default_impl! { usize, 0, "Returns the default value of `0`" }
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { u8, 0, "Returns the default value of `0`" }
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { u16, 0, "Returns the default value of `0`" }
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { u32, 0, "Returns the default value of `0`" }
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { u64, 0, "Returns the default value of `0`" }
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { u128, 0, "Returns the default value of `0`" }
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
default_impl! { isize, 0, "Returns the default value of `0`" }
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { i8, 0, "Returns the default value of `0`" }
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { i16, 0, "Returns the default value of `0`" }
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { i32, 0, "Returns the default value of `0`" }
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { i64, 0, "Returns the default value of `0`" }
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { i128, 0, "Returns the default value of `0`" }
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
default_impl! { f16, 0.0f16, "Returns the default value of `0.0`" }
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { f32, 0.0f32, "Returns the default value of `0.0`" }
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { f64, 0.0f64, "Returns the default value of `0.0`" }
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
default_impl! { f128, 0.0f128, "Returns the default value of `0.0`" }
```

---
*Generated by AST tracing system*
