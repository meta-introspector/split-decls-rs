# AST Trace: ../rust/library/core/src/fmt/mod.rs

Generated 77 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=5

```rust
//! Utilities for formatting and printing strings.

#![stable(feature = "rust1", since = "1.0.0")]

use crate::cell::{Cell, Ref, RefCell, RefMut, SyncUnsafeCell, UnsafeCell};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::char::{EscapeDebugExtArgs, MAX_LEN_UTF8};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::marker::{PhantomData, PointeeSized};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::num::fmt as numfmt;
use crate::ops::Deref;
use crate::{iter, result, str};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=25

```rust
mod builders;
#[cfg(not(no_fp_fmt_parse))]
mod float;
#[cfg(no_fp_fmt_parse)]
mod nofloat;
mod num;
mod num_buffer;
mod rt;

#[stable(feature = "fmt_flags_align", since = "1.28.0")]
#[rustc_diagnostic_item = "Alignment"]
/// Possible alignments returned by `Formatter::align`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Alignment {
    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    /// Indication that contents should be left-aligned.
    Left,
    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    /// Indication that contents should be right-aligned.
    Right,
    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    /// Indication that contents should be center-aligned.
    Center,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[unstable(feature = "int_format_into", issue = "138215")]
pub use num_buffer::{NumBuffer, NumBufferTrait};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
#[stable(feature = "debug_builders", since = "1.2.0")]
pub use self::builders::{DebugList, DebugMap, DebugSet, DebugStruct, DebugTuple};
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
#[unstable(feature = "debug_closure_helpers", issue = "117729")]
pub use self::builders::{FromFn, from_fn};
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
/// The type returned by formatter methods.
///
/// # Examples
///
/// ```
/// use std::fmt;
///
/// #[derive(Debug)]
/// struct Triangle {
///     a: f32,
///     b: f32,
///     c: f32
/// }
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=8 | LINES=6

```rust
///
/// impl fmt::Display for Triangle {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "({}, {}, {})", self.a, self.b, self.c)
///     }
/// }
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
///
/// let pythagorean_triple = Triangle { a: 3.0, b: 4.0, c: 5.0 };
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=5 | LINES=33

```rust
///
/// assert_eq!(format!("{pythagorean_triple}"), "(3, 4, 5)");
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub type Result = result::Result<(), Error>;

/// The error type which is returned from formatting a message into a stream.
///
/// This type does not support transmission of an error other than that an error
/// occurred. This is because, despite the existence of this error,
/// string formatting is considered an infallible operation.
/// `fmt()` implementors should not return this `Error` unless they received it from their
/// [`Formatter`]. The only time your code should create a new instance of this
/// error is when implementing `fmt::Write`, in order to cancel the formatting operation when
/// writing to the underlying stream fails.
///
/// Any extra information must be arranged to be transmitted through some other means,
/// such as storing it in a field to be consulted after the formatting operation has been
/// cancelled. (For example, this is how [`std::io::Write::write_fmt()`] propagates IO errors
/// during writing.)
///
/// This type, `fmt::Error`, should not be
/// confused with [`std::io::Error`] or [`std::error::Error`], which you may also
/// have in scope.
///
/// [`std::io::Error`]: ../../std/io/struct.Error.html
/// [`std::io::Write::write_fmt()`]: ../../std/io/trait.Write.html#method.write_fmt
/// [`std::error::Error`]: ../../std/error/trait.Error.html
///
/// # Examples
///
/// ```rust
/// use std::fmt::{self, write};
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=5 | LINES=5

```rust
///
/// let mut output = String::new();
/// if let Err(fmt::Error) = write(&mut output, format_args!("Hello {}!", "world")) {
///     panic!("An error occurred");
/// }
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=Error; | COMPLEXITY=49 | LINES=137

```rust
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

/// A trait for writing or formatting into Unicode-accepting buffers or streams.
///
/// This trait only accepts UTF-8–encoded data and is not [flushable]. If you only
/// want to accept Unicode and you don't need flushing, you should implement this trait;
/// otherwise you should implement [`std::io::Write`].
///
/// [`std::io::Write`]: ../../std/io/trait.Write.html
/// [flushable]: ../../std/io/trait.Write.html#tymethod.flush
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Write {
    /// Writes a string slice into this writer, returning whether the write
    /// succeeded.
    ///
    /// This method can only succeed if the entire string slice was successfully
    /// written, and this method will not return until all data has been
    /// written or an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an instance of [`std::fmt::Error`][Error] on error.
    ///
    /// The purpose of that error is to abort the formatting operation when the underlying
    /// destination encounters some error preventing it from accepting more text;
    /// in particular, it does not communicate any information about *what* error occurred.
    /// It should generally be propagated rather than handled, at least when implementing
    /// formatting traits.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt::{Error, Write};
    ///
    /// fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    ///     f.write_str(s)
    /// }
    ///
    /// let mut buf = String::new();
    /// writer(&mut buf, "hola")?;
    /// assert_eq!(&buf, "hola");
    /// # std::fmt::Result::Ok(())
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn write_str(&mut self, s: &str) -> Result;

    /// Writes a [`char`] into this writer, returning whether the write succeeded.
    ///
    /// A single [`char`] may be encoded as more than one byte.
    /// This method can only succeed if the entire byte sequence was successfully
    /// written, and this method will not return until all data has been
    /// written or an error occurs.
    ///
    /// # Errors
    ///
    /// This function will return an instance of [`Error`] on error.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt::{Error, Write};
    ///
    /// fn writer<W: Write>(f: &mut W, c: char) -> Result<(), Error> {
    ///     f.write_char(c)
    /// }
    ///
    /// let mut buf = String::new();
    /// writer(&mut buf, 'a')?;
    /// writer(&mut buf, 'b')?;
    /// assert_eq!(&buf, "ab");
    /// # std::fmt::Result::Ok(())
    /// ```
    #[stable(feature = "fmt_write_char", since = "1.1.0")]
    fn write_char(&mut self, c: char) -> Result {
        self.write_str(c.encode_utf8(&mut [0; MAX_LEN_UTF8]))
    }

    /// Glue for usage of the [`write!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through
    /// the [`write!`] macro itself.
    ///
    /// # Errors
    ///
    /// This function will return an instance of [`Error`] on error. Please see
    /// [write_str](Write::write_str) for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt::{Error, Write};
    ///
    /// fn writer<W: Write>(f: &mut W, s: &str) -> Result<(), Error> {
    ///     f.write_fmt(format_args!("{s}"))
    /// }
    ///
    /// let mut buf = String::new();
    /// writer(&mut buf, "world")?;
    /// assert_eq!(&buf, "world");
    /// # std::fmt::Result::Ok(())
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        // We use a specialization for `Sized` types to avoid an indirection
        // through `&mut self`
        trait SpecWriteFmt {
            fn spec_write_fmt(self, args: Arguments<'_>) -> Result;
        }

        impl<W: Write + ?Sized> SpecWriteFmt for &mut W {
            #[inline]
            default fn spec_write_fmt(mut self, args: Arguments<'_>) -> Result {
                if let Some(s) = args.as_statically_known_str() {
                    self.write_str(s)
                } else {
                    write(&mut self, args)
                }
            }
        }

        impl<W: Write> SpecWriteFmt for &mut W {
            #[inline]
            fn spec_write_fmt(self, args: Arguments<'_>) -> Result {
                if let Some(s) = args.as_statically_known_str() {
                    self.write_str(s)
                } else {
                    write(self, args)
                }
            }
        }

        self.spec_write_fmt(args)
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=write_str | COMPLEXITY=7 | LINES=15

```rust
#[stable(feature = "fmt_write_blanket_impl", since = "1.4.0")]
impl<W: Write + ?Sized> Write for &mut W {
    fn write_str(&mut self, s: &str) -> Result {
        (**self).write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result {
        (**self).write_char(c)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        (**self).write_fmt(args)
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
/// The signedness of a [`Formatter`] (or of a [`FormattingOptions`]).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[unstable(feature = "formatting_options", issue = "118117")]
pub enum Sign {
    /// Represents the `+` flag.
    Plus,
    /// Represents the `-` flag.
    Minus,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=11

```rust
/// Specifies whether the [`Debug`] trait should use lower-/upper-case
/// hexadecimal or normal integers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[unstable(feature = "formatting_options", issue = "118117")]
pub enum DebugAsHex {
    /// Use lower-case hexadecimal integers for the `Debug` trait (like [the `x?` type](../../std/fmt/index.html#formatting-traits)).
    Lower,
    /// Use upper-case hexadecimal integers for the `Debug` trait (like [the `X?` type](../../std/fmt/index.html#formatting-traits)).
    Upper,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=STRUCT | NAME=FormattingOptions | COMPLEXITY=14 | LINES=37

```rust
/// Options for formatting.
///
/// `FormattingOptions` is a [`Formatter`] without an attached [`Write`] trait.
/// It is mainly used to construct `Formatter` instances.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[unstable(feature = "formatting_options", issue = "118117")]
pub struct FormattingOptions {
    /// Flags, with the following bit fields:
    ///
    /// ```text
    ///   31  30  29  28  27  26  25  24  23  22  21  20                              0
    /// ┌───┬───────┬───┬───┬───┬───┬───┬───┬───┬───┬──────────────────────────────────┐
    /// │ 1 │ align │ p │ w │ X?│ x?│'0'│ # │ - │ + │               fill               │
    /// └───┴───────┴───┴───┴───┴───┴───┴───┴───┴───┴──────────────────────────────────┘
    ///   │     │     │   │  └─┬───────────────────┘ └─┬──────────────────────────────┘
    ///   │     │     │   │    │                       └─ The fill character (21 bits char).
    ///   │     │     │   │    └─ The debug upper/lower hex, zero pad, alternate, and plus/minus flags.
    ///   │     │     │   └─ Whether a width is set. (The value is stored separately.)
    ///   │     │     └─ Whether a precision is set. (The value is stored separately.)
    ///   │     ├─ 0: Align left. (<)
    ///   │     ├─ 1: Align right. (>)
    ///   │     ├─ 2: Align center. (^)
    ///   │     └─ 3: Alignment not set. (default)
    ///   └─ Always set.
    ///      This makes it possible to distinguish formatting flags from
    ///      a &str size when stored in (the upper bits of) the same field.
    ///      (fmt::Arguments will make use of this property in the future.)
    /// ```
    // Note: This could use a special niche type with range 0x8000_0000..=0xfdd0ffff.
    // It's unclear if that's useful, though.
    flags: u32,
    /// Width if width flag (bit 27) above is set. Otherwise, always 0.
    width: u16,
    /// Precision if precision flag (bit 28) above is set. Otherwise, always 0.
    precision: u16,
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=MODULE | NAME=UNNAMED | COMPLEXITY=6 | LINES=18

```rust
// This needs to match with compiler/rustc_ast_lowering/src/format.rs.
mod flags {
    pub(super) const SIGN_PLUS_FLAG: u32 = 1 << 21;
    pub(super) const SIGN_MINUS_FLAG: u32 = 1 << 22;
    pub(super) const ALTERNATE_FLAG: u32 = 1 << 23;
    pub(super) const SIGN_AWARE_ZERO_PAD_FLAG: u32 = 1 << 24;
    pub(super) const DEBUG_LOWER_HEX_FLAG: u32 = 1 << 25;
    pub(super) const DEBUG_UPPER_HEX_FLAG: u32 = 1 << 26;
    pub(super) const WIDTH_FLAG: u32 = 1 << 27;
    pub(super) const PRECISION_FLAG: u32 = 1 << 28;
    pub(super) const ALIGN_BITS: u32 = 0b11 << 29;
    pub(super) const ALIGN_LEFT: u32 = 0 << 29;
    pub(super) const ALIGN_RIGHT: u32 = 1 << 29;
    pub(super) const ALIGN_CENTER: u32 = 2 << 29;
    pub(super) const ALIGN_UNKNOWN: u32 = 3 << 29;
    pub(super) const ALWAYS_SET: u32 = 1 << 31;
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=114 | LINES=212

```rust
impl FormattingOptions {
    /// Construct a new `FormatterBuilder` with the supplied `Write` trait
    /// object for output that is equivalent to the `{}` formatting
    /// specifier:
    ///
    /// - no flags,
    /// - filled with spaces,
    /// - no alignment,
    /// - no width,
    /// - no precision, and
    /// - no [`DebugAsHex`] output mode.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn new() -> Self {
        Self {
            flags: ' ' as u32 | flags::ALIGN_UNKNOWN | flags::ALWAYS_SET,
            width: 0,
            precision: 0,
        }
    }

    /// Sets or removes the sign (the `+` or the `-` flag).
    ///
    /// - `+`: This is intended for numeric types and indicates that the sign
    ///   should always be printed. By default only the negative sign of signed
    ///   values is printed, and the sign of positive or unsigned values is
    ///   omitted. This flag indicates that the correct sign (+ or -) should
    ///   always be printed.
    /// - `-`: Currently not used
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn sign(&mut self, sign: Option<Sign>) -> &mut Self {
        let sign = match sign {
            None => 0,
            Some(Sign::Plus) => flags::SIGN_PLUS_FLAG,
            Some(Sign::Minus) => flags::SIGN_MINUS_FLAG,
        };
        self.flags = self.flags & !(flags::SIGN_PLUS_FLAG | flags::SIGN_MINUS_FLAG) | sign;
        self
    }
    /// Sets or unsets the `0` flag.
    ///
    /// This is used to indicate for integer formats that the padding to width should both be done with a 0 character as well as be sign-aware
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn sign_aware_zero_pad(&mut self, sign_aware_zero_pad: bool) -> &mut Self {
        if sign_aware_zero_pad {
            self.flags |= flags::SIGN_AWARE_ZERO_PAD_FLAG;
        } else {
            self.flags &= !flags::SIGN_AWARE_ZERO_PAD_FLAG;
        }
        self
    }
    /// Sets or unsets the `#` flag.
    ///
    /// This flag indicates that the "alternate" form of printing should be
    /// used. The alternate forms are:
    /// - [`Debug`] : pretty-print the [`Debug`] formatting (adds linebreaks and indentation)
    /// - [`LowerHex`] as well as [`UpperHex`] - precedes the argument with a `0x`
    /// - [`Octal`] - precedes the argument with a `0b`
    /// - [`Binary`] - precedes the argument with a `0o`
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn alternate(&mut self, alternate: bool) -> &mut Self {
        if alternate {
            self.flags |= flags::ALTERNATE_FLAG;
        } else {
            self.flags &= !flags::ALTERNATE_FLAG;
        }
        self
    }
    /// Sets the fill character.
    ///
    /// The optional fill character and alignment is provided normally in
    /// conjunction with the width parameter. This indicates that if the value
    /// being formatted is smaller than width some extra characters will be
    /// printed around it.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn fill(&mut self, fill: char) -> &mut Self {
        self.flags = self.flags & (u32::MAX << 21) | fill as u32;
        self
    }
    /// Sets or removes the alignment.
    ///
    /// The alignment specifies how the value being formatted should be
    /// positioned if it is smaller than the width of the formatter.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn align(&mut self, align: Option<Alignment>) -> &mut Self {
        let align: u32 = match align {
            Some(Alignment::Left) => flags::ALIGN_LEFT,
            Some(Alignment::Right) => flags::ALIGN_RIGHT,
            Some(Alignment::Center) => flags::ALIGN_CENTER,
            None => flags::ALIGN_UNKNOWN,
        };
        self.flags = self.flags & !flags::ALIGN_BITS | align;
        self
    }
    /// Sets or removes the width.
    ///
    /// This is a parameter for the “minimum width” that the format should take
    /// up. If the value’s string does not fill up this many characters, then
    /// the padding specified by [`FormattingOptions::fill`]/[`FormattingOptions::align`]
    /// will be used to take up the required space.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn width(&mut self, width: Option<u16>) -> &mut Self {
        if let Some(width) = width {
            self.flags |= flags::WIDTH_FLAG;
            self.width = width;
        } else {
            self.flags &= !flags::WIDTH_FLAG;
            self.width = 0;
        }
        self
    }
    /// Sets or removes the precision.
    ///
    /// - For non-numeric types, this can be considered a “maximum width”. If
    ///   the resulting string is longer than this width, then it is truncated
    ///   down to this many characters and that truncated value is emitted with
    ///   proper fill, alignment and width if those parameters are set.
    /// - For integral types, this is ignored.
    /// - For floating-point types, this indicates how many digits after the
    /// decimal point should be printed.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn precision(&mut self, precision: Option<u16>) -> &mut Self {
        if let Some(precision) = precision {
            self.flags |= flags::PRECISION_FLAG;
            self.precision = precision;
        } else {
            self.flags &= !flags::PRECISION_FLAG;
            self.precision = 0;
        }
        self
    }
    /// Specifies whether the [`Debug`] trait should use lower-/upper-case
    /// hexadecimal or normal integers
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn debug_as_hex(&mut self, debug_as_hex: Option<DebugAsHex>) -> &mut Self {
        let debug_as_hex = match debug_as_hex {
            None => 0,
            Some(DebugAsHex::Lower) => flags::DEBUG_LOWER_HEX_FLAG,
            Some(DebugAsHex::Upper) => flags::DEBUG_UPPER_HEX_FLAG,
        };
        self.flags = self.flags & !(flags::DEBUG_LOWER_HEX_FLAG | flags::DEBUG_UPPER_HEX_FLAG)
            | debug_as_hex;
        self
    }

    /// Returns the current sign (the `+` or the `-` flag).
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_sign(&self) -> Option<Sign> {
        if self.flags & flags::SIGN_PLUS_FLAG != 0 {
            Some(Sign::Plus)
        } else if self.flags & flags::SIGN_MINUS_FLAG != 0 {
            Some(Sign::Minus)
        } else {
            None
        }
    }
    /// Returns the current `0` flag.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_sign_aware_zero_pad(&self) -> bool {
        self.flags & flags::SIGN_AWARE_ZERO_PAD_FLAG != 0
    }
    /// Returns the current `#` flag.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_alternate(&self) -> bool {
        self.flags & flags::ALTERNATE_FLAG != 0
    }
    /// Returns the current fill character.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_fill(&self) -> char {
        // SAFETY: We only ever put a valid `char` in the lower 21 bits of the flags field.
        unsafe { char::from_u32_unchecked(self.flags & 0x1FFFFF) }
    }
    /// Returns the current alignment.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_align(&self) -> Option<Alignment> {
        match self.flags & flags::ALIGN_BITS {
            flags::ALIGN_LEFT => Some(Alignment::Left),
            flags::ALIGN_RIGHT => Some(Alignment::Right),
            flags::ALIGN_CENTER => Some(Alignment::Center),
            _ => None,
        }
    }
    /// Returns the current width.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_width(&self) -> Option<u16> {
        if self.flags & flags::WIDTH_FLAG != 0 { Some(self.width) } else { None }
    }
    /// Returns the current precision.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_precision(&self) -> Option<u16> {
        if self.flags & flags::PRECISION_FLAG != 0 { Some(self.precision) } else { None }
    }
    /// Returns the current precision.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn get_debug_as_hex(&self) -> Option<DebugAsHex> {
        if self.flags & flags::DEBUG_LOWER_HEX_FLAG != 0 {
            Some(DebugAsHex::Lower)
        } else if self.flags & flags::DEBUG_UPPER_HEX_FLAG != 0 {
            Some(DebugAsHex::Upper)
        } else {
            None
        }
    }

    /// Creates a [`Formatter`] that writes its output to the given [`Write`] trait.
    ///
    /// You may alternatively use [`Formatter::new()`].
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn create_formatter<'a>(self, write: &'a mut (dyn Write + 'a)) -> Formatter<'a> {
        Formatter { options: self, buf: write }
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=default | COMPLEXITY=5 | LINES=9

```rust
#[unstable(feature = "formatting_options", issue = "118117")]
impl Default for FormattingOptions {
    /// Same as [`FormattingOptions::new()`].
    fn default() -> Self {
        // The `#[derive(Default)]` implementation would set `fill` to `\0` instead of space.
        Self::new()
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=STRUCT | NAME=Formatter | COMPLEXITY=5 | LINES=18

```rust
/// Configuration for formatting.
///
/// A `Formatter` represents various options related to formatting. Users do not
/// construct `Formatter`s directly; a mutable reference to one is passed to
/// the `fmt` method of all formatting traits, like [`Debug`] and [`Display`].
///
/// To interact with a `Formatter`, you'll call various methods to change the
/// various options related to formatting. For examples, please see the
/// documentation of the methods defined on `Formatter` below.
#[allow(missing_debug_implementations)]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "Formatter"]
pub struct Formatter<'a> {
    options: FormattingOptions,

    buf: &'a mut (dyn Write + 'a),
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=20

```rust
impl<'a> Formatter<'a> {
    /// Creates a new formatter with given [`FormattingOptions`].
    ///
    /// If `write` is a reference to a formatter, it is recommended to use
    /// [`Formatter::with_options`] instead as this can borrow the underlying
    /// `write`, thereby bypassing one layer of indirection.
    ///
    /// You may alternatively use [`FormattingOptions::create_formatter()`].
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn new(write: &'a mut (dyn Write + 'a), options: FormattingOptions) -> Self {
        Formatter { options, buf: write }
    }

    /// Creates a new formatter based on this one with given [`FormattingOptions`].
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn with_options<'b>(&'b mut self, options: FormattingOptions) -> Formatter<'b> {
        Formatter { options, buf: self.buf }
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=STRUCT | NAME=Arguments | COMPLEXITY=17 | LINES=37

```rust
/// This structure represents a safely precompiled version of a format string
/// and its arguments. This cannot be generated at runtime because it cannot
/// safely be done, so no constructors are given and the fields are private
/// to prevent modification.
///
/// The [`format_args!`] macro will safely create an instance of this structure.
/// The macro validates the format string at compile-time so usage of the
/// [`write()`] and [`format()`] functions can be safely performed.
///
/// You can use the `Arguments<'a>` that [`format_args!`] returns in `Debug`
/// and `Display` contexts as seen below. The example also shows that `Debug`
/// and `Display` format to the same thing: the interpolated format string
/// in `format_args!`.
///
/// ```rust
/// let debug = format!("{:?}", format_args!("{} foo {:?}", 1, 2));
/// let display = format!("{}", format_args!("{} foo {:?}", 1, 2));
/// assert_eq!("1 foo 2", display);
/// assert_eq!(display, debug);
/// ```
///
/// [`format()`]: ../../std/fmt/fn.format.html
#[lang = "format_arguments"]
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Copy, Clone)]
pub struct Arguments<'a> {
    // Format string pieces to print.
    pieces: &'a [&'static str],

    // Placeholder specs, or `None` if all specs are default (as in "{}{}").
    fmt: Option<&'a [rt::Placeholder]>,

    // Dynamic arguments for interpolation, to be interleaved with string
    // pieces. (Every argument is preceded by a string piece.)
    args: &'a [rt::Argument<'a>],
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=estimated_capacity | COMPLEXITY=14 | LINES=27

```rust
#[doc(hidden)]
#[unstable(feature = "fmt_internals", issue = "none")]
impl<'a> Arguments<'a> {
    /// Estimates the length of the formatted text.
    ///
    /// This is intended to be used for setting initial `String` capacity
    /// when using `format!`. Note: this is neither the lower nor upper bound.
    #[inline]
    pub fn estimated_capacity(&self) -> usize {
        let pieces_length: usize = self.pieces.iter().map(|x| x.len()).sum();

        if self.args.is_empty() {
            pieces_length
        } else if !self.pieces.is_empty() && self.pieces[0].is_empty() && pieces_length < 16 {
            // If the format string starts with an argument,
            // don't preallocate anything, unless length
            // of pieces is significant.
            0
        } else {
            // There are some arguments, so any additional push
            // will reallocate the string. To avoid that,
            // we're "pre-doubling" the capacity here.
            pieces_length.checked_mul(2).unwrap_or(0)
        }
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=as_statically_known_str | COMPLEXITY=34 | LINES=66

```rust
impl<'a> Arguments<'a> {
    /// Gets the formatted string, if it has no arguments to be formatted at runtime.
    ///
    /// This can be used to avoid allocations in some cases.
    ///
    /// # Guarantees
    ///
    /// For `format_args!("just a literal")`, this function is guaranteed to
    /// return `Some("just a literal")`.
    ///
    /// For most cases with placeholders, this function will return `None`.
    ///
    /// However, the compiler may perform optimizations that can cause this
    /// function to return `Some(_)` even if the format string contains
    /// placeholders. For example, `format_args!("Hello, {}!", "world")` may be
    /// optimized to `format_args!("Hello, world!")`, such that `as_str()`
    /// returns `Some("Hello, world!")`.
    ///
    /// The behavior for anything but the trivial case (without placeholders)
    /// is not guaranteed, and should not be relied upon for anything other
    /// than optimization.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt::Arguments;
    ///
    /// fn write_str(_: &str) { /* ... */ }
    ///
    /// fn write_fmt(args: &Arguments<'_>) {
    ///     if let Some(s) = args.as_str() {
    ///         write_str(s)
    ///     } else {
    ///         write_str(&args.to_string());
    ///     }
    /// }
    /// ```
    ///
    /// ```rust
    /// assert_eq!(format_args!("hello").as_str(), Some("hello"));
    /// assert_eq!(format_args!("").as_str(), Some(""));
    /// assert_eq!(format_args!("{:?}", std::env::current_dir()).as_str(), None);
    /// ```
    #[stable(feature = "fmt_as_str", since = "1.52.0")]
    #[rustc_const_stable(feature = "const_arguments_as_str", since = "1.84.0")]
    #[must_use]
    #[inline]
    pub const fn as_str(&self) -> Option<&'static str> {
        match (self.pieces, self.args) {
            ([], []) => Some(""),
            ([s], []) => Some(s),
            _ => None,
        }
    }

    /// Same as [`Arguments::as_str`], but will only return `Some(s)` if it can be determined at compile time.
    #[unstable(feature = "fmt_internals", reason = "internal to standard library", issue = "none")]
    #[must_use]
    #[inline]
    #[doc(hidden)]
    pub fn as_statically_known_str(&self) -> Option<&'static str> {
        let s = self.as_str();
        if core::intrinsics::is_val_statically_known(s.is_some()) { s } else { None }
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=4

```rust
// Manually implementing these results in better error messages.
#[stable(feature = "rust1", since = "1.0.0")]
impl !Send for Arguments<'_> {}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=2

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl !Sync for Arguments<'_> {}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for Arguments<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        Display::fmt(self, fmt)
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl Display for Arguments<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        write(fmt.buf, *self)
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=7 | LINES=16

```rust
/// `?` formatting.
///
/// `Debug` should format the output in a programmer-facing, debugging context.
///
/// Generally speaking, you should just `derive` a `Debug` implementation.
///
/// When used with the alternate format specifier `#?`, the output is pretty-printed.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
///
/// This trait can be used with `#[derive]` if all fields implement `Debug`. When
/// `derive`d for structs, it will use the name of the `struct`, then `{`, then a
/// comma-separated list of each field's name and `Debug` value, then `}`. For
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=20

```rust
/// `enum`s, it will use the name of the variant and, if applicable, `(`, then the
/// `Debug` values of the fields, then `)`.
///
/// # Stability
///
/// Derived `Debug` formats are not stable, and so may change with future Rust
/// versions. Additionally, `Debug` implementations of types provided by the
/// standard library (`std`, `core`, `alloc`, etc.) are not stable, and
/// may also change with future Rust versions.
///
/// # Examples
///
/// Deriving an implementation:
///
/// ```
/// #[derive(Debug)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
///
/// let origin = Point { x: 0, y: 0 };
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=4 | LINES=16

```rust
///
/// assert_eq!(
///     format!("The origin is: {origin:?}"),
///     "The origin is: Point { x: 0, y: 0 }",
/// );
/// ```
///
/// Manually implementing:
///
/// ```
/// use std::fmt;
///
/// struct Point {
///     x: i32,
///     y: i32,
/// }
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=9

```rust
///
/// impl fmt::Debug for Point {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         f.debug_struct("Point")
///          .field("x", &self.x)
///          .field("y", &self.y)
///          .finish()
///     }
/// }
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
///
/// let origin = Point { x: 0, y: 0 };
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=23

```rust
///
/// assert_eq!(
///     format!("The origin is: {origin:?}"),
///     "The origin is: Point { x: 0, y: 0 }",
/// );
/// ```
///
/// There are a number of helper methods on the [`Formatter`] struct to help you with manual
/// implementations, such as [`debug_struct`].
///
/// [`debug_struct`]: Formatter::debug_struct
///
/// Types that do not wish to use the standard suite of debug representations
/// provided by the `Formatter` trait (`debug_struct`, `debug_tuple`,
/// `debug_list`, `debug_set`, `debug_map`) can do something totally custom by
/// manually writing an arbitrary representation to the `Formatter`.
///
/// ```
/// # use std::fmt;
/// # struct Point {
/// #     x: i32,
/// #     y: i32,
/// # }
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=6

```rust
/// #
/// impl fmt::Debug for Point {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "Point [{} {}]", self.x, self.y)
///     }
/// }
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
/// ```
///
/// `Debug` implementations using either `derive` or the debug builder API
/// on [`Formatter`] support pretty-printing using the alternate flag: `{:#?}`.
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
///
/// Pretty-printing with `#?`:
///
/// ```
/// #[derive(Debug)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
///
/// let origin = Point { x: 0, y: 0 };
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=27 | LINES=55

```rust
///
/// let expected = "The origin is: Point {
///     x: 0,
///     y: 0,
/// }";
/// assert_eq!(format!("The origin is: {origin:#?}"), expected);
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_on_unimplemented(
    on(
        crate_local,
        note = "add `#[derive(Debug)]` to `{Self}` or manually `impl {This} for {Self}`"
    ),
    on(
        from_desugaring = "FormatLiteral",
        label = "`{Self}` cannot be formatted using `{{:?}}` because it doesn't implement `{This}`"
    ),
    message = "`{Self}` doesn't implement `{This}`"
)]
#[doc(alias = "{:?}")]
#[rustc_diagnostic_item = "Debug"]
#[rustc_trivial_field_reads]
pub trait Debug: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Position {
    ///     longitude: f32,
    ///     latitude: f32,
    /// }
    ///
    /// impl fmt::Debug for Position {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         f.debug_tuple("")
    ///          .field(&self.longitude)
    ///          .field(&self.latitude)
    ///          .finish()
    ///     }
    /// }
    ///
    /// let position = Position { longitude: 1.987, latitude: 2.983 };
    /// assert_eq!(format!("{position:?}"), "(1.987, 2.983)");
    ///
    /// assert_eq!(format!("{position:#?}"), "(
    ///     1.987,
    ///     2.983,
    /// )");
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
// Separate module to reexport the macro `Debug` from prelude without the trait `Debug`.
pub(crate) mod macros {
    /// Derive macro generating an impl of the trait `Debug`.
    #[rustc_builtin_macro]
    #[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
    #[allow_internal_unstable(core_intrinsics, fmt_helpers_for_derive)]
    pub macro Debug($item:item) {
        /* compiler built-in */
    }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=5

```rust
#[stable(feature = "builtin_macro_prelude", since = "1.38.0")]
#[doc(inline)]
pub use macros::Debug;

/// Format trait for an empty format, `{}`.
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=32 | LINES=57

```rust
///
/// Implementing this trait for a type will automatically implement the
/// [`ToString`][tostring] trait for the type, allowing the usage
/// of the [`.to_string()`][tostring_function] method. Prefer implementing
/// the `Display` trait for a type, rather than [`ToString`][tostring].
///
/// `Display` is similar to [`Debug`], but `Display` is for user-facing
/// output, and so cannot be derived.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
/// [tostring]: ../../std/string/trait.ToString.html
/// [tostring_function]: ../../std/string/trait.ToString.html#tymethod.to_string
///
/// # Completeness and parseability
///
/// `Display` for a type might not necessarily be a lossless or complete representation of the type.
/// It may omit internal state, precision, or other information the type does not consider important
/// for user-facing output, as determined by the type. As such, the output of `Display` might not be
/// possible to parse, and even if it is, the result of parsing might not exactly match the original
/// value.
///
/// However, if a type has a lossless `Display` implementation whose output is meant to be
/// conveniently machine-parseable and not just meant for human consumption, then the type may wish
/// to accept the same format in `FromStr`, and document that usage. Having both `Display` and
/// `FromStr` implementations where the result of `Display` cannot be parsed with `FromStr` may
/// surprise users.
///
/// # Internationalization
///
/// Because a type can only have one `Display` implementation, it is often preferable
/// to only implement `Display` when there is a single most "obvious" way that
/// values can be formatted as text. This could mean formatting according to the
/// "invariant" culture and "undefined" locale, or it could mean that the type
/// display is designed for a specific culture/locale, such as developer logs.
///
/// If not all values have a justifiably canonical textual format or if you want
/// to support alternative formats not covered by the standard set of possible
/// [formatting traits], the most flexible approach is display adapters: methods
/// like [`str::escape_default`] or [`Path::display`] which create a wrapper
/// implementing `Display` to output the specific display format.
///
/// [formatting traits]: ../../std/fmt/index.html#formatting-traits
/// [`Path::display`]: ../../std/path/struct.Path.html#method.display
///
/// # Examples
///
/// Implementing `Display` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Point {
///     x: i32,
///     y: i32,
/// }
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=6

```rust
///
/// impl fmt::Display for Point {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "({}, {})", self.x, self.y)
///     }
/// }
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
///
/// let origin = Point { x: 0, y: 0 };
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=26 | LINES=47

```rust
///
/// assert_eq!(format!("The origin is: {origin}"), "The origin is: (0, 0)");
/// ```
#[rustc_on_unimplemented(
    on(
        any(Self = "std::path::Path", Self = "std::path::PathBuf"),
        label = "`{Self}` cannot be formatted with the default formatter; call `.display()` on it",
        note = "call `.display()` or `.to_string_lossy()` to safely print paths, \
                as they may contain non-Unicode data",
    ),
    on(
        from_desugaring = "FormatLiteral",
        note = "in format strings you may be able to use `{{:?}}` (or {{:#?}} for pretty-print) instead",
        label = "`{Self}` cannot be formatted with the default formatter",
    ),
    message = "`{Self}` doesn't implement `{This}`"
)]
#[doc(alias = "{}")]
#[rustc_diagnostic_item = "Display"]
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Display: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Position {
    ///     longitude: f32,
    ///     latitude: f32,
    /// }
    ///
    /// impl fmt::Display for Position {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "({}, {})", self.longitude, self.latitude)
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "(1.987, 2.983)",
    ///     format!("{}", Position { longitude: 1.987, latitude: 2.983, }),
    /// );
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=10 | LINES=41

```rust
/// `o` formatting.
///
/// The `Octal` trait should format its output as a number in base-8.
///
/// For primitive signed integers (`i8` to `i128`, and `isize`),
/// negative values are formatted as the two’s complement representation.
///
/// The alternate flag, `#`, adds a `0o` in front of the output.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
///
/// # Examples
///
/// Basic usage with `i32`:
///
/// ```
/// let x = 42; // 42 is '52' in octal
///
/// assert_eq!(format!("{x:o}"), "52");
/// assert_eq!(format!("{x:#o}"), "0o52");
///
/// assert_eq!(format!("{:o}", -16), "37777777760");
/// ```
///
/// Implementing `Octal` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Length(i32);
///
/// impl fmt::Octal for Length {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         let val = self.0;
///
///         fmt::Octal::fmt(&val, f) // delegate to i32's implementation
///     }
/// }
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=4 | LINES=13

```rust
///
/// let l = Length(9);
///
/// assert_eq!(format!("l as octal is: {l:o}"), "l as octal is: 11");
///
/// assert_eq!(format!("l as octal is: {l:#06o}"), "l as octal is: 0o0011");
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Octal: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=10 | LINES=41

```rust
/// `b` formatting.
///
/// The `Binary` trait should format its output as a number in binary.
///
/// For primitive signed integers ([`i8`] to [`i128`], and [`isize`]),
/// negative values are formatted as the two’s complement representation.
///
/// The alternate flag, `#`, adds a `0b` in front of the output.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
///
/// # Examples
///
/// Basic usage with [`i32`]:
///
/// ```
/// let x = 42; // 42 is '101010' in binary
///
/// assert_eq!(format!("{x:b}"), "101010");
/// assert_eq!(format!("{x:#b}"), "0b101010");
///
/// assert_eq!(format!("{:b}", -16), "11111111111111111111111111110000");
/// ```
///
/// Implementing `Binary` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Length(i32);
///
/// impl fmt::Binary for Length {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         let val = self.0;
///
///         fmt::Binary::fmt(&val, f) // delegate to i32's implementation
///     }
/// }
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=18

```rust
///
/// let l = Length(107);
///
/// assert_eq!(format!("l as binary is: {l:b}"), "l as binary is: 1101011");
///
/// assert_eq!(
///     // Note that the `0b` prefix added by `#` is included in the total width, so we
///     // need to add two to correctly display all 32 bits.
///     format!("l as binary is: {l:#034b}"),
///     "l as binary is: 0b00000000000000000000000001101011"
/// );
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Binary: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=10 | LINES=42

```rust
/// `x` formatting.
///
/// The `LowerHex` trait should format its output as a number in hexadecimal, with `a` through `f`
/// in lower case.
///
/// For primitive signed integers (`i8` to `i128`, and `isize`),
/// negative values are formatted as the two’s complement representation.
///
/// The alternate flag, `#`, adds a `0x` in front of the output.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
///
/// # Examples
///
/// Basic usage with `i32`:
///
/// ```
/// let y = 42; // 42 is '2a' in hex
///
/// assert_eq!(format!("{y:x}"), "2a");
/// assert_eq!(format!("{y:#x}"), "0x2a");
///
/// assert_eq!(format!("{:x}", -16), "fffffff0");
/// ```
///
/// Implementing `LowerHex` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Length(i32);
///
/// impl fmt::LowerHex for Length {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         let val = self.0;
///
///         fmt::LowerHex::fmt(&val, f) // delegate to i32's implementation
///     }
/// }
```

## Block 54
**Metadata**: AST_ID=54 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=4 | LINES=13

```rust
///
/// let l = Length(9);
///
/// assert_eq!(format!("l as hex is: {l:x}"), "l as hex is: 9");
///
/// assert_eq!(format!("l as hex is: {l:#010x}"), "l as hex is: 0x00000009");
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub trait LowerHex: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 55
**Metadata**: AST_ID=55 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=10 | LINES=42

```rust
/// `X` formatting.
///
/// The `UpperHex` trait should format its output as a number in hexadecimal, with `A` through `F`
/// in upper case.
///
/// For primitive signed integers (`i8` to `i128`, and `isize`),
/// negative values are formatted as the two’s complement representation.
///
/// The alternate flag, `#`, adds a `0x` in front of the output.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
///
/// # Examples
///
/// Basic usage with `i32`:
///
/// ```
/// let y = 42; // 42 is '2A' in hex
///
/// assert_eq!(format!("{y:X}"), "2A");
/// assert_eq!(format!("{y:#X}"), "0x2A");
///
/// assert_eq!(format!("{:X}", -16), "FFFFFFF0");
/// ```
///
/// Implementing `UpperHex` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Length(i32);
///
/// impl fmt::UpperHex for Length {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         let val = self.0;
///
///         fmt::UpperHex::fmt(&val, f) // delegate to i32's implementation
///     }
/// }
```

## Block 56
**Metadata**: AST_ID=56 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=4 | LINES=13

```rust
///
/// let l = Length(i32::MAX);
///
/// assert_eq!(format!("l as hex is: {l:X}"), "l as hex is: 7FFFFFFF");
///
/// assert_eq!(format!("l as hex is: {l:#010X}"), "l as hex is: 0x7FFFFFFF");
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub trait UpperHex: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 57
**Metadata**: AST_ID=57 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=10 | LINES=43

```rust
/// `p` formatting.
///
/// The `Pointer` trait should format its output as a memory location. This is commonly presented
/// as hexadecimal. For more information on formatters, see [the module-level documentation][module].
///
/// Printing of pointers is not a reliable way to discover how Rust programs are implemented.
/// The act of reading an address changes the program itself, and may change how the data is represented
/// in memory, and may affect which optimizations are applied to the code.
///
/// The printed pointer values are not guaranteed to be stable nor unique identifiers of objects.
/// Rust allows moving values to different memory locations, and may reuse the same memory locations
/// for different purposes.
///
/// There is no guarantee that the printed value can be converted back to a pointer.
///
/// [module]: ../../std/fmt/index.html
///
/// # Examples
///
/// Basic usage with `&i32`:
///
/// ```
/// let x = &42;
///
/// let address = format!("{x:p}"); // this produces something like '0x7f06092ac6d0'
/// ```
///
/// Implementing `Pointer` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Length(i32);
///
/// impl fmt::Pointer for Length {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         // use `as` to convert to a `*const T`, which implements Pointer, which we can use
///
///         let ptr = self as *const Self;
///         fmt::Pointer::fmt(&ptr, f)
///     }
/// }
```

## Block 58
**Metadata**: AST_ID=58 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=4 | LINES=16

```rust
///
/// let l = Length(42);
///
/// println!("l is in memory here: {l:p}");
///
/// let l_ptr = format!("{l:018p}");
/// assert_eq!(l_ptr.len(), 18);
/// assert_eq!(&l_ptr[..2], "0x");
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "Pointer"]
pub trait Pointer: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 59
**Metadata**: AST_ID=59 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=32

```rust
/// `e` formatting.
///
/// The `LowerExp` trait should format its output in scientific notation with a lower-case `e`.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
///
/// # Examples
///
/// Basic usage with `f64`:
///
/// ```
/// let x = 42.0; // 42.0 is '4.2e1' in scientific notation
///
/// assert_eq!(format!("{x:e}"), "4.2e1");
/// ```
///
/// Implementing `LowerExp` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Length(i32);
///
/// impl fmt::LowerExp for Length {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         let val = f64::from(self.0);
///         fmt::LowerExp::fmt(&val, f) // delegate to f64's implementation
///     }
/// }
```

## Block 60
**Metadata**: AST_ID=60 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=19

```rust
///
/// let l = Length(100);
///
/// assert_eq!(
///     format!("l in scientific notation is: {l:e}"),
///     "l in scientific notation is: 1e2"
/// );
///
/// assert_eq!(
///     format!("l in scientific notation is: {l:05e}"),
///     "l in scientific notation is: 001e2"
/// );
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub trait LowerExp: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 61
**Metadata**: AST_ID=61 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=32

```rust
/// `E` formatting.
///
/// The `UpperExp` trait should format its output in scientific notation with an upper-case `E`.
///
/// For more information on formatters, see [the module-level documentation][module].
///
/// [module]: ../../std/fmt/index.html
///
/// # Examples
///
/// Basic usage with `f64`:
///
/// ```
/// let x = 42.0; // 42.0 is '4.2E1' in scientific notation
///
/// assert_eq!(format!("{x:E}"), "4.2E1");
/// ```
///
/// Implementing `UpperExp` on a type:
///
/// ```
/// use std::fmt;
///
/// struct Length(i32);
///
/// impl fmt::UpperExp for Length {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         let val = f64::from(self.0);
///         fmt::UpperExp::fmt(&val, f) // delegate to f64's implementation
///     }
/// }
```

## Block 62
**Metadata**: AST_ID=62 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=19

```rust
///
/// let l = Length(100);
///
/// assert_eq!(
///     format!("l in scientific notation is: {l:E}"),
///     "l in scientific notation is: 1E2"
/// );
///
/// assert_eq!(
///     format!("l in scientific notation is: {l:05E}"),
///     "l in scientific notation is: 001E2"
/// );
/// ```
#[stable(feature = "rust1", since = "1.0.0")]
pub trait UpperExp: PointeeSized {
    #[doc = include_str!("fmt_trait_method_doc.md")]
    #[stable(feature = "rust1", since = "1.0.0")]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

## Block 63
**Metadata**: AST_ID=63 | TYPE=FUNCTION | NAME=write | COMPLEXITY=56 | LINES=81

```rust
/// Takes an output stream and an `Arguments` struct that can be precompiled with
/// the `format_args!` macro.
///
/// The arguments will be formatted according to the specified format string
/// into the output stream provided.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::fmt;
///
/// let mut output = String::new();
/// fmt::write(&mut output, format_args!("Hello {}!", "world"))
///     .expect("Error occurred while trying to write in String");
/// assert_eq!(output, "Hello world!");
/// ```
///
/// Please note that using [`write!`] might be preferable. Example:
///
/// ```
/// use std::fmt::Write;
///
/// let mut output = String::new();
/// write!(&mut output, "Hello {}!", "world")
///     .expect("Error occurred while trying to write in String");
/// assert_eq!(output, "Hello world!");
/// ```
///
/// [`write!`]: crate::write!
#[stable(feature = "rust1", since = "1.0.0")]
pub fn write(output: &mut dyn Write, args: Arguments<'_>) -> Result {
    let mut formatter = Formatter::new(output, FormattingOptions::new());
    let mut idx = 0;

    match args.fmt {
        None => {
            // We can use default formatting parameters for all arguments.
            for (i, arg) in args.args.iter().enumerate() {
                // SAFETY: args.args and args.pieces come from the same Arguments,
                // which guarantees the indexes are always within bounds.
                let piece = unsafe { args.pieces.get_unchecked(i) };
                if !piece.is_empty() {
                    formatter.buf.write_str(*piece)?;
                }

                // SAFETY: There are no formatting parameters and hence no
                // count arguments.
                unsafe {
                    arg.fmt(&mut formatter)?;
                }
                idx += 1;
            }
        }
        Some(fmt) => {
            // Every spec has a corresponding argument that is preceded by
            // a string piece.
            for (i, arg) in fmt.iter().enumerate() {
                // SAFETY: fmt and args.pieces come from the same Arguments,
                // which guarantees the indexes are always within bounds.
                let piece = unsafe { args.pieces.get_unchecked(i) };
                if !piece.is_empty() {
                    formatter.buf.write_str(*piece)?;
                }
                // SAFETY: arg and args.args come from the same Arguments,
                // which guarantees the indexes are always within bounds.
                unsafe { run(&mut formatter, arg, args.args) }?;
                idx += 1;
            }
        }
    }

    // There can be only one trailing string piece left.
    if let Some(piece) = args.pieces.get(idx) {
        formatter.buf.write_str(*piece)?;
    }

    Ok(())
}
```

## Block 64
**Metadata**: AST_ID=64 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=23 | LINES=22

```rust
unsafe fn run(fmt: &mut Formatter<'_>, arg: &rt::Placeholder, args: &[rt::Argument<'_>]) -> Result {
    let (width, precision) =
        // SAFETY: arg and args come from the same Arguments,
        // which guarantees the indexes are always within bounds.
        unsafe { (getcount(args, &arg.width), getcount(args, &arg.precision)) };

    let options = FormattingOptions { flags: arg.flags, width, precision };

    // Extract the correct argument
    debug_assert!(arg.position < args.len());
    // SAFETY: arg and args come from the same Arguments,
    // which guarantees its index is always within bounds.
    let value = unsafe { args.get_unchecked(arg.position) };

    // Set all the formatting options.
    fmt.options = options;

    // Then actually do some printing
    // SAFETY: this is a placeholder argument.
    unsafe { value.fmt(fmt) }
}
```

## Block 65
**Metadata**: AST_ID=65 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=16 | LINES=13

```rust
unsafe fn getcount(args: &[rt::Argument<'_>], cnt: &rt::Count) -> u16 {
    match *cnt {
        rt::Count::Is(n) => n,
        rt::Count::Implied => 0,
        rt::Count::Param(i) => {
            debug_assert!(i < args.len());
            // SAFETY: cnt and args come from the same Arguments,
            // which guarantees this index is always within bounds.
            unsafe { args.get_unchecked(i).as_u16().unwrap_unchecked() }
        }
    }
}
```

## Block 66
**Metadata**: AST_ID=66 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
/// Padding after the end of something. Returned by `Formatter::padding`.
#[must_use = "don't forget to write the post padding"]
pub(crate) struct PostPadding {
    fill: char,
    padding: u16,
}
```

## Block 67
**Metadata**: AST_ID=67 | TYPE=FUNCTION | NAME=new | COMPLEXITY=8 | LINES=14

```rust
impl PostPadding {
    fn new(fill: char, padding: u16) -> PostPadding {
        PostPadding { fill, padding }
    }

    /// Writes this post padding.
    pub(crate) fn write(self, f: &mut Formatter<'_>) -> Result {
        for _ in 0..self.padding {
            f.buf.write_char(self.fill)?;
        }
        Ok(())
    }
}
```

## Block 68
**Metadata**: AST_ID=68 | TYPE=FUNCTION | NAME=wrap_buf | COMPLEXITY=546 | LINES=1069

```rust
impl<'a> Formatter<'a> {
    fn wrap_buf<'b, 'c, F>(&'b mut self, wrap: F) -> Formatter<'c>
    where
        'b: 'c,
        F: FnOnce(&'b mut (dyn Write + 'b)) -> &'c mut (dyn Write + 'c),
    {
        Formatter {
            // We want to change this
            buf: wrap(self.buf),

            // And preserve these
            options: self.options,
        }
    }

    // Helper methods used for padding and processing formatting arguments that
    // all formatting traits can use.

    /// Performs the correct padding for an integer which has already been
    /// emitted into a str. The str should *not* contain the sign for the
    /// integer, that will be added by this method.
    ///
    /// # Arguments
    ///
    /// * is_nonnegative - whether the original integer was either positive or zero.
    /// * prefix - if the '#' character (Alternate) is provided, this
    ///   is the prefix to put in front of the number.
    /// * buf - the byte array that the number has been formatted into
    ///
    /// This function will correctly account for the flags provided as well as
    /// the minimum width. It will not take precision into account.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo { nb: i32 }
    ///
    /// impl Foo {
    ///     fn new(nb: i32) -> Foo {
    ///         Foo {
    ///             nb,
    ///         }
    ///     }
    /// }
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         // We need to remove "-" from the number output.
    ///         let tmp = self.nb.abs().to_string();
    ///
    ///         formatter.pad_integral(self.nb >= 0, "Foo ", &tmp)
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{}", Foo::new(2)), "2");
    /// assert_eq!(format!("{}", Foo::new(-1)), "-1");
    /// assert_eq!(format!("{}", Foo::new(0)), "0");
    /// assert_eq!(format!("{:#}", Foo::new(-1)), "-Foo 1");
    /// assert_eq!(format!("{:0>#8}", Foo::new(-1)), "00-Foo 1");
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pad_integral(&mut self, is_nonnegative: bool, prefix: &str, buf: &str) -> Result {
        let mut width = buf.len();

        let mut sign = None;
        if !is_nonnegative {
            sign = Some('-');
            width += 1;
        } else if self.sign_plus() {
            sign = Some('+');
            width += 1;
        }

        let prefix = if self.alternate() {
            width += prefix.chars().count();
            Some(prefix)
        } else {
            None
        };

        // Writes the sign if it exists, and then the prefix if it was requested
        #[inline(never)]
        fn write_prefix(f: &mut Formatter<'_>, sign: Option<char>, prefix: Option<&str>) -> Result {
            if let Some(c) = sign {
                f.buf.write_char(c)?;
            }
            if let Some(prefix) = prefix { f.buf.write_str(prefix) } else { Ok(()) }
        }

        // The `width` field is more of a `min-width` parameter at this point.
        let min = self.options.width;
        if width >= usize::from(min) {
            // We're over the minimum width, so then we can just write the bytes.
            write_prefix(self, sign, prefix)?;
            self.buf.write_str(buf)
        } else if self.sign_aware_zero_pad() {
            // The sign and prefix goes before the padding if the fill character
            // is zero
            let old_options = self.options;
            self.options.fill('0').align(Some(Alignment::Right));
            write_prefix(self, sign, prefix)?;
            let post_padding = self.padding(min - width as u16, Alignment::Right)?;
            self.buf.write_str(buf)?;
            post_padding.write(self)?;
            self.options = old_options;
            Ok(())
        } else {
            // Otherwise, the sign and prefix goes after the padding
            let post_padding = self.padding(min - width as u16, Alignment::Right)?;
            write_prefix(self, sign, prefix)?;
            self.buf.write_str(buf)?;
            post_padding.write(self)
        }
    }

    /// Takes a string slice and emits it to the internal buffer after applying
    /// the relevant formatting flags specified.
    ///
    /// The flags recognized for generic strings are:
    ///
    /// * width - the minimum width of what to emit
    /// * fill/align - what to emit and where to emit it if the string
    ///                provided needs to be padded
    /// * precision - the maximum length to emit, the string is truncated if it
    ///               is longer than this length
    ///
    /// Notably this function ignores the `flag` parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo;
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         formatter.pad("Foo")
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{Foo:<4}"), "Foo ");
    /// assert_eq!(format!("{Foo:0>4}"), "0Foo");
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pad(&mut self, s: &str) -> Result {
        // Make sure there's a fast path up front.
        if self.options.flags & (flags::WIDTH_FLAG | flags::PRECISION_FLAG) == 0 {
            return self.buf.write_str(s);
        }

        // The `precision` field can be interpreted as a maximum width for the
        // string being formatted.
        let (s, char_count) = if let Some(max_char_count) = self.options.get_precision() {
            let mut iter = s.char_indices();
            let remaining = match iter.advance_by(usize::from(max_char_count)) {
                Ok(()) => 0,
                Err(remaining) => remaining.get(),
            };
            // SAFETY: The offset of `.char_indices()` is guaranteed to be
            // in-bounds and between character boundaries.
            let truncated = unsafe { s.get_unchecked(..iter.offset()) };
            (truncated, usize::from(max_char_count) - remaining)
        } else {
            // Use the optimized char counting algorithm for the full string.
            (s, s.chars().count())
        };

        // The `width` field is more of a minimum width parameter at this point.
        if char_count < usize::from(self.options.width) {
            // If we're under the minimum width, then fill up the minimum width
            // with the specified string + some alignment.
            let post_padding =
                self.padding(self.options.width - char_count as u16, Alignment::Left)?;
            self.buf.write_str(s)?;
            post_padding.write(self)
        } else {
            // If we're over the minimum width or there is no minimum width, we
            // can just emit the string.
            self.buf.write_str(s)
        }
    }

    /// Writes the pre-padding and returns the unwritten post-padding.
    ///
    /// Callers are responsible for ensuring post-padding is written after the
    /// thing that is being padded.
    pub(crate) fn padding(
        &mut self,
        padding: u16,
        default: Alignment,
    ) -> result::Result<PostPadding, Error> {
        let align = self.options.get_align().unwrap_or(default);
        let fill = self.options.get_fill();

        let padding_left = match align {
            Alignment::Left => 0,
            Alignment::Right => padding,
            Alignment::Center => padding / 2,
        };

        for _ in 0..padding_left {
            self.buf.write_char(fill)?;
        }

        Ok(PostPadding::new(fill, padding - padding_left))
    }

    /// Takes the formatted parts and applies the padding.
    ///
    /// Assumes that the caller already has rendered the parts with required precision,
    /// so that `self.precision` can be ignored.
    ///
    /// # Safety
    ///
    /// Any `numfmt::Part::Copy` parts in `formatted` must contain valid UTF-8.
    unsafe fn pad_formatted_parts(&mut self, formatted: &numfmt::Formatted<'_>) -> Result {
        if self.options.width == 0 {
            // this is the common case and we take a shortcut
            // SAFETY: Per the precondition.
            unsafe { self.write_formatted_parts(formatted) }
        } else {
            // for the sign-aware zero padding, we render the sign first and
            // behave as if we had no sign from the beginning.
            let mut formatted = formatted.clone();
            let mut width = self.options.width;
            let old_options = self.options;
            if self.sign_aware_zero_pad() {
                // a sign always goes first
                let sign = formatted.sign;
                self.buf.write_str(sign)?;

                // remove the sign from the formatted parts
                formatted.sign = "";
                width = width.saturating_sub(sign.len() as u16);
                self.options.fill('0').align(Some(Alignment::Right));
            }

            // remaining parts go through the ordinary padding process.
            let len = formatted.len();
            let ret = if usize::from(width) <= len {
                // no padding
                // SAFETY: Per the precondition.
                unsafe { self.write_formatted_parts(&formatted) }
            } else {
                let post_padding = self.padding(width - len as u16, Alignment::Right)?;
                // SAFETY: Per the precondition.
                unsafe {
                    self.write_formatted_parts(&formatted)?;
                }
                post_padding.write(self)
            };
            self.options = old_options;
            ret
        }
    }

    /// # Safety
    ///
    /// Any `numfmt::Part::Copy` parts in `formatted` must contain valid UTF-8.
    unsafe fn write_formatted_parts(&mut self, formatted: &numfmt::Formatted<'_>) -> Result {
        unsafe fn write_bytes(buf: &mut dyn Write, s: &[u8]) -> Result {
            // SAFETY: This is used for `numfmt::Part::Num` and `numfmt::Part::Copy`.
            // It's safe to use for `numfmt::Part::Num` since every char `c` is between
            // `b'0'` and `b'9'`, which means `s` is valid UTF-8. It's safe to use for
            // `numfmt::Part::Copy` due to this function's precondition.
            buf.write_str(unsafe { str::from_utf8_unchecked(s) })
        }

        if !formatted.sign.is_empty() {
            self.buf.write_str(formatted.sign)?;
        }
        for part in formatted.parts {
            match *part {
                numfmt::Part::Zero(mut nzeroes) => {
                    const ZEROES: &str = // 64 zeroes
                        "0000000000000000000000000000000000000000000000000000000000000000";
                    while nzeroes > ZEROES.len() {
                        self.buf.write_str(ZEROES)?;
                        nzeroes -= ZEROES.len();
                    }
                    if nzeroes > 0 {
                        self.buf.write_str(&ZEROES[..nzeroes])?;
                    }
                }
                numfmt::Part::Num(mut v) => {
                    let mut s = [0; 5];
                    let len = part.len();
                    for c in s[..len].iter_mut().rev() {
                        *c = b'0' + (v % 10) as u8;
                        v /= 10;
                    }
                    // SAFETY: Per the precondition.
                    unsafe {
                        write_bytes(self.buf, &s[..len])?;
                    }
                }
                // SAFETY: Per the precondition.
                numfmt::Part::Copy(buf) => unsafe {
                    write_bytes(self.buf, buf)?;
                },
            }
        }
        Ok(())
    }

    /// Writes some data to the underlying buffer contained within this
    /// formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo;
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         formatter.write_str("Foo")
    ///         // This is equivalent to:
    ///         // write!(formatter, "Foo")
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{Foo}"), "Foo");
    /// assert_eq!(format!("{Foo:0>8}"), "Foo");
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn write_str(&mut self, data: &str) -> Result {
        self.buf.write_str(data)
    }

    /// Glue for usage of the [`write!`] macro with implementors of this trait.
    ///
    /// This method should generally not be invoked manually, but rather through
    /// the [`write!`] macro itself.
    ///
    /// Writes some formatted information into this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(i32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         formatter.write_fmt(format_args!("Foo {}", self.0))
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{}", Foo(-1)), "Foo -1");
    /// assert_eq!(format!("{:0>8}", Foo(2)), "Foo 2");
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    #[inline]
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result {
        if let Some(s) = fmt.as_statically_known_str() {
            self.buf.write_str(s)
        } else {
            write(self.buf, fmt)
        }
    }

    /// Returns flags for formatting.
    #[must_use]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[deprecated(
        since = "1.24.0",
        note = "use the `sign_plus`, `sign_minus`, `alternate`, \
                or `sign_aware_zero_pad` methods instead"
    )]
    pub fn flags(&self) -> u32 {
        // Extract the debug upper/lower hex, zero pad, alternate, and plus/minus flags
        // to stay compatible with older versions of Rust.
        self.options.flags >> 21 & 0x3F
    }

    /// Returns the character used as 'fill' whenever there is alignment.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo;
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         let c = formatter.fill();
    ///         if let Some(width) = formatter.width() {
    ///             for _ in 0..width {
    ///                 write!(formatter, "{c}")?;
    ///             }
    ///             Ok(())
    ///         } else {
    ///             write!(formatter, "{c}")
    ///         }
    ///     }
    /// }
    ///
    /// // We set alignment to the right with ">".
    /// assert_eq!(format!("{Foo:G>3}"), "GGG");
    /// assert_eq!(format!("{Foo:t>6}"), "tttttt");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn fill(&self) -> char {
        self.options.get_fill()
    }

    /// Returns a flag indicating what form of alignment was requested.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt::{self, Alignment};
    ///
    /// struct Foo;
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         let s = if let Some(s) = formatter.align() {
    ///             match s {
    ///                 Alignment::Left    => "left",
    ///                 Alignment::Right   => "right",
    ///                 Alignment::Center  => "center",
    ///             }
    ///         } else {
    ///             "into the void"
    ///         };
    ///         write!(formatter, "{s}")
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{Foo:<}"), "left");
    /// assert_eq!(format!("{Foo:>}"), "right");
    /// assert_eq!(format!("{Foo:^}"), "center");
    /// assert_eq!(format!("{Foo}"), "into the void");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags_align", since = "1.28.0")]
    pub fn align(&self) -> Option<Alignment> {
        self.options.get_align()
    }

    /// Returns the optionally specified integer width that the output should be.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(i32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         if let Some(width) = formatter.width() {
    ///             // If we received a width, we use it
    ///             write!(formatter, "{:width$}", format!("Foo({})", self.0), width = width)
    ///         } else {
    ///             // Otherwise we do nothing special
    ///             write!(formatter, "Foo({})", self.0)
    ///         }
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:10}", Foo(23)), "Foo(23)   ");
    /// assert_eq!(format!("{}", Foo(23)), "Foo(23)");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn width(&self) -> Option<usize> {
        if self.options.flags & flags::WIDTH_FLAG == 0 {
            None
        } else {
            Some(self.options.width as usize)
        }
    }

    /// Returns the optionally specified precision for numeric types.
    /// Alternatively, the maximum width for string types.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(f32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         if let Some(precision) = formatter.precision() {
    ///             // If we received a precision, we use it.
    ///             write!(formatter, "Foo({1:.*})", precision, self.0)
    ///         } else {
    ///             // Otherwise we default to 2.
    ///             write!(formatter, "Foo({:.2})", self.0)
    ///         }
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:.4}", Foo(23.2)), "Foo(23.2000)");
    /// assert_eq!(format!("{}", Foo(23.2)), "Foo(23.20)");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn precision(&self) -> Option<usize> {
        if self.options.flags & flags::PRECISION_FLAG == 0 {
            None
        } else {
            Some(self.options.precision as usize)
        }
    }

    /// Determines if the `+` flag was specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(i32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         if formatter.sign_plus() {
    ///             write!(formatter,
    ///                    "Foo({}{})",
    ///                    if self.0 < 0 { '-' } else { '+' },
    ///                    self.0.abs())
    ///         } else {
    ///             write!(formatter, "Foo({})", self.0)
    ///         }
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:+}", Foo(23)), "Foo(+23)");
    /// assert_eq!(format!("{:+}", Foo(-23)), "Foo(-23)");
    /// assert_eq!(format!("{}", Foo(23)), "Foo(23)");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_plus(&self) -> bool {
        self.options.flags & flags::SIGN_PLUS_FLAG != 0
    }

    /// Determines if the `-` flag was specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(i32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         if formatter.sign_minus() {
    ///             // You want a minus sign? Have one!
    ///             write!(formatter, "-Foo({})", self.0)
    ///         } else {
    ///             write!(formatter, "Foo({})", self.0)
    ///         }
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:-}", Foo(23)), "-Foo(23)");
    /// assert_eq!(format!("{}", Foo(23)), "Foo(23)");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_minus(&self) -> bool {
        self.options.flags & flags::SIGN_MINUS_FLAG != 0
    }

    /// Determines if the `#` flag was specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(i32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         if formatter.alternate() {
    ///             write!(formatter, "Foo({})", self.0)
    ///         } else {
    ///             write!(formatter, "{}", self.0)
    ///         }
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:#}", Foo(23)), "Foo(23)");
    /// assert_eq!(format!("{}", Foo(23)), "23");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn alternate(&self) -> bool {
        self.options.flags & flags::ALTERNATE_FLAG != 0
    }

    /// Determines if the `0` flag was specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fmt;
    ///
    /// struct Foo(i32);
    ///
    /// impl fmt::Display for Foo {
    ///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         assert!(formatter.sign_aware_zero_pad());
    ///         assert_eq!(formatter.width(), Some(4));
    ///         // We ignore the formatter's options.
    ///         write!(formatter, "{}", self.0)
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:04}", Foo(23)), "23");
    /// ```
    #[must_use]
    #[stable(feature = "fmt_flags", since = "1.5.0")]
    pub fn sign_aware_zero_pad(&self) -> bool {
        self.options.flags & flags::SIGN_AWARE_ZERO_PAD_FLAG != 0
    }

    // FIXME: Decide what public API we want for these two flags.
    // https://github.com/rust-lang/rust/issues/48584
    fn debug_lower_hex(&self) -> bool {
        self.options.flags & flags::DEBUG_LOWER_HEX_FLAG != 0
    }
    fn debug_upper_hex(&self) -> bool {
        self.options.flags & flags::DEBUG_UPPER_HEX_FLAG != 0
    }

    /// Creates a [`DebugStruct`] builder designed to assist with creation of
    /// [`fmt::Debug`] implementations for structs.
    ///
    /// [`fmt::Debug`]: self::Debug
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    /// use std::net::Ipv4Addr;
    ///
    /// struct Foo {
    ///     bar: i32,
    ///     baz: String,
    ///     addr: Ipv4Addr,
    /// }
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_struct("Foo")
    ///             .field("bar", &self.bar)
    ///             .field("baz", &self.baz)
    ///             .field("addr", &format_args!("{}", self.addr))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo { bar: 10, baz: \"Hello World\", addr: 127.0.0.1 }",
    ///     format!("{:?}", Foo {
    ///         bar: 10,
    ///         baz: "Hello World".to_string(),
    ///         addr: Ipv4Addr::new(127, 0, 0, 1),
    ///     })
    /// );
    /// ```
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_struct<'b>(&'b mut self, name: &str) -> DebugStruct<'b, 'a> {
        builders::debug_struct_new(self, name)
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_struct_fields_finish` is more general, but this is
    /// faster for 1 field.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_struct_field1_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str,
        value1: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_struct_new(self, name);
        builder.field(name1, value1);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_struct_fields_finish` is more general, but this is
    /// faster for 2 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_struct_field2_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str,
        value1: &dyn Debug,
        name2: &str,
        value2: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_struct_new(self, name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_struct_fields_finish` is more general, but this is
    /// faster for 3 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_struct_field3_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str,
        value1: &dyn Debug,
        name2: &str,
        value2: &dyn Debug,
        name3: &str,
        value3: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_struct_new(self, name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.field(name3, value3);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_struct_fields_finish` is more general, but this is
    /// faster for 4 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_struct_field4_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str,
        value1: &dyn Debug,
        name2: &str,
        value2: &dyn Debug,
        name3: &str,
        value3: &dyn Debug,
        name4: &str,
        value4: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_struct_new(self, name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.field(name3, value3);
        builder.field(name4, value4);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_struct_fields_finish` is more general, but this is
    /// faster for 5 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_struct_field5_finish<'b>(
        &'b mut self,
        name: &str,
        name1: &str,
        value1: &dyn Debug,
        name2: &str,
        value2: &dyn Debug,
        name3: &str,
        value3: &dyn Debug,
        name4: &str,
        value4: &dyn Debug,
        name5: &str,
        value5: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_struct_new(self, name);
        builder.field(name1, value1);
        builder.field(name2, value2);
        builder.field(name3, value3);
        builder.field(name4, value4);
        builder.field(name5, value5);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller binaries.
    /// For the cases not covered by `debug_struct_field[12345]_finish`.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_struct_fields_finish<'b>(
        &'b mut self,
        name: &str,
        names: &[&str],
        values: &[&dyn Debug],
    ) -> Result {
        assert_eq!(names.len(), values.len());
        let mut builder = builders::debug_struct_new(self, name);
        for (name, value) in iter::zip(names, values) {
            builder.field(name, value);
        }
        builder.finish()
    }

    /// Creates a `DebugTuple` builder designed to assist with creation of
    /// `fmt::Debug` implementations for tuple structs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    /// use std::marker::PhantomData;
    ///
    /// struct Foo<T>(i32, String, PhantomData<T>);
    ///
    /// impl<T> fmt::Debug for Foo<T> {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_tuple("Foo")
    ///             .field(&self.0)
    ///             .field(&self.1)
    ///             .field(&format_args!("_"))
    ///             .finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     "Foo(10, \"Hello\", _)",
    ///     format!("{:?}", Foo(10, "Hello".to_string(), PhantomData::<u8>))
    /// );
    /// ```
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_tuple<'b>(&'b mut self, name: &str) -> DebugTuple<'b, 'a> {
        builders::debug_tuple_new(self, name)
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_tuple_fields_finish` is more general, but this is faster
    /// for 1 field.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_tuple_field1_finish<'b>(&'b mut self, name: &str, value1: &dyn Debug) -> Result {
        let mut builder = builders::debug_tuple_new(self, name);
        builder.field(value1);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_tuple_fields_finish` is more general, but this is faster
    /// for 2 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_tuple_field2_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_tuple_new(self, name);
        builder.field(value1);
        builder.field(value2);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_tuple_fields_finish` is more general, but this is faster
    /// for 3 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_tuple_field3_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
        value3: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_tuple_new(self, name);
        builder.field(value1);
        builder.field(value2);
        builder.field(value3);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_tuple_fields_finish` is more general, but this is faster
    /// for 4 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_tuple_field4_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
        value3: &dyn Debug,
        value4: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_tuple_new(self, name);
        builder.field(value1);
        builder.field(value2);
        builder.field(value3);
        builder.field(value4);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. `debug_tuple_fields_finish` is more general, but this is faster
    /// for 5 fields.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_tuple_field5_finish<'b>(
        &'b mut self,
        name: &str,
        value1: &dyn Debug,
        value2: &dyn Debug,
        value3: &dyn Debug,
        value4: &dyn Debug,
        value5: &dyn Debug,
    ) -> Result {
        let mut builder = builders::debug_tuple_new(self, name);
        builder.field(value1);
        builder.field(value2);
        builder.field(value3);
        builder.field(value4);
        builder.field(value5);
        builder.finish()
    }

    /// Shrinks `derive(Debug)` code, for faster compilation and smaller
    /// binaries. For the cases not covered by `debug_tuple_field[12345]_finish`.
    #[doc(hidden)]
    #[unstable(feature = "fmt_helpers_for_derive", issue = "none")]
    pub fn debug_tuple_fields_finish<'b>(
        &'b mut self,
        name: &str,
        values: &[&dyn Debug],
    ) -> Result {
        let mut builder = builders::debug_tuple_new(self, name);
        for value in values {
            builder.field(value);
        }
        builder.finish()
    }

    /// Creates a `DebugList` builder designed to assist with creation of
    /// `fmt::Debug` implementations for list-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_list().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", Foo(vec![10, 11])), "[10, 11]");
    /// ```
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_list<'b>(&'b mut self) -> DebugList<'b, 'a> {
        builders::debug_list_new(self)
    }

    /// Creates a `DebugSet` builder designed to assist with creation of
    /// `fmt::Debug` implementations for set-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<i32>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_set().entries(self.0.iter()).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", Foo(vec![10, 11])), "{10, 11}");
    /// ```
    ///
    /// [`format_args!`]: crate::format_args
    ///
    /// In this more complex example, we use [`format_args!`] and `.debug_set()`
    /// to build a list of match arms:
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Arm<'a, L, R>(&'a (L, R));
    /// struct Table<'a, K, V>(&'a [(K, V)], V);
    ///
    /// impl<'a, L, R> fmt::Debug for Arm<'a, L, R>
    /// where
    ///     L: 'a + fmt::Debug, R: 'a + fmt::Debug
    /// {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         L::fmt(&(self.0).0, fmt)?;
    ///         fmt.write_str(" => ")?;
    ///         R::fmt(&(self.0).1, fmt)
    ///     }
    /// }
    ///
    /// impl<'a, K, V> fmt::Debug for Table<'a, K, V>
    /// where
    ///     K: 'a + fmt::Debug, V: 'a + fmt::Debug
    /// {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_set()
    ///         .entries(self.0.iter().map(Arm))
    ///         .entry(&Arm(&(format_args!("_"), &self.1)))
    ///         .finish()
    ///     }
    /// }
    /// ```
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_set<'b>(&'b mut self) -> DebugSet<'b, 'a> {
        builders::debug_set_new(self)
    }

    /// Creates a `DebugMap` builder designed to assist with creation of
    /// `fmt::Debug` implementations for map-like structures.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::fmt;
    ///
    /// struct Foo(Vec<(String, i32)>);
    ///
    /// impl fmt::Debug for Foo {
    ///     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         fmt.debug_map().entries(self.0.iter().map(|&(ref k, ref v)| (k, v))).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(
    ///     format!("{:?}",  Foo(vec![("A".to_string(), 10), ("B".to_string(), 11)])),
    ///     r#"{"A": 10, "B": 11}"#
    ///  );
    /// ```
    #[stable(feature = "debug_builders", since = "1.2.0")]
    pub fn debug_map<'b>(&'b mut self) -> DebugMap<'b, 'a> {
        builders::debug_map_new(self)
    }

    /// Returns the sign of this formatter (`+` or `-`).
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn sign(&self) -> Option<Sign> {
        self.options.get_sign()
    }

    /// Returns the formatting options this formatter corresponds to.
    #[unstable(feature = "formatting_options", issue = "118117")]
    pub const fn options(&self) -> FormattingOptions {
        self.options
    }
}
```

## Block 69
**Metadata**: AST_ID=69 | TYPE=FUNCTION | NAME=write_str | COMPLEXITY=11 | LINES=20

```rust
#[stable(since = "1.2.0", feature = "formatter_write")]
impl Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> Result {
        self.buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> Result {
        self.buf.write_char(c)
    }

    #[inline]
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        if let Some(s) = args.as_statically_known_str() {
            self.buf.write_str(s)
        } else {
            write(self.buf, args)
        }
    }
}
```

## Block 70
**Metadata**: AST_ID=70 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt("an error occurred when formatting an argument", f)
    }
}
```

## Block 71
**Metadata**: AST_ID=71 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=17 | LINES=17

```rust
// Implementations of the core formatting traits

macro_rules! fmt_refs {
    ($($tr:ident),*) => {
        $(
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: PointeeSized + $tr> $tr for &T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
        }
        #[stable(feature = "rust1", since = "1.0.0")]
        impl<T: PointeeSized + $tr> $tr for &mut T {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result { $tr::fmt(&**self, f) }
        }
        )*
    }
}
```

## Block 72
**Metadata**: AST_ID=72 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }
```

## Block 73
**Metadata**: AST_ID=73 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=8

```rust
#[unstable(feature = "never_type", issue = "35121")]
impl Debug for ! {
    #[inline]
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        *self
    }
}
```

## Block 74
**Metadata**: AST_ID=74 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=8

```rust
#[unstable(feature = "never_type", issue = "35121")]
impl Display for ! {
    #[inline]
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        *self
    }
}
```

## Block 75
**Metadata**: AST_ID=75 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for bool {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self, f)
    }
}
```

## Block 76
**Metadata**: AST_ID=76 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=9 | LINES=7

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl Display for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(if *self { "true" } else { "false" }, f)
    }
}
```

## Block 77
**Metadata**: AST_ID=77 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=172 | LINES=277

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('"')?;

        // substring we know is printable
        let mut printable_range = 0..0;

        fn needs_escape(b: u8) -> bool {
            b > 0x7E || b < 0x20 || b == b'\\' || b == b'"'
        }

        // the loop here first skips over runs of printable ASCII as a fast path.
        // other chars (unicode, or ASCII that needs escaping) are then handled per-`char`.
        let mut rest = self;
        while rest.len() > 0 {
            let Some(non_printable_start) = rest.as_bytes().iter().position(|&b| needs_escape(b))
            else {
                printable_range.end += rest.len();
                break;
            };

            printable_range.end += non_printable_start;
            // SAFETY: the position was derived from an iterator, so is known to be within bounds, and at a char boundary
            rest = unsafe { rest.get_unchecked(non_printable_start..) };

            let mut chars = rest.chars();
            if let Some(c) = chars.next() {
                let esc = c.escape_debug_ext(EscapeDebugExtArgs {
                    escape_grapheme_extended: true,
                    escape_single_quote: false,
                    escape_double_quote: true,
                });
                if esc.len() != 1 {
                    f.write_str(&self[printable_range.clone()])?;
                    Display::fmt(&esc, f)?;
                    printable_range.start = printable_range.end + c.len_utf8();
                }
                printable_range.end += c.len_utf8();
            }
            rest = chars.as_str();
        }

        f.write_str(&self[printable_range])?;

        f.write_char('"')
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad(self)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char('\'')?;
        let esc = self.escape_debug_ext(EscapeDebugExtArgs {
            escape_grapheme_extended: true,
            escape_single_quote: true,
            escape_double_quote: false,
        });
        Display::fmt(&esc, f)?;
        f.write_char('\'')
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Display for char {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.options.flags & (flags::WIDTH_FLAG | flags::PRECISION_FLAG) == 0 {
            f.write_char(*self)
        } else {
            f.pad(self.encode_utf8(&mut [0; MAX_LEN_UTF8]))
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PointeeSized> Pointer for *const T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if <<T as core::ptr::Pointee>::Metadata as core::unit::IsUnit>::is_unit() {
            pointer_fmt_inner(self.expose_provenance(), f)
        } else {
            f.debug_struct("Pointer")
                .field_with("addr", |f| pointer_fmt_inner(self.expose_provenance(), f))
                .field("metadata", &core::ptr::metadata(*self))
                .finish()
        }
    }
}

/// Since the formatting will be identical for all pointer types, uses a
/// non-monomorphized implementation for the actual formatting to reduce the
/// amount of codegen work needed.
///
/// This uses `ptr_addr: usize` and not `ptr: *const ()` to be able to use this for
/// `fn(...) -> ...` without using [problematic] "Oxford Casts".
///
/// [problematic]: https://github.com/rust-lang/rust/issues/95489
pub(crate) fn pointer_fmt_inner(ptr_addr: usize, f: &mut Formatter<'_>) -> Result {
    let old_options = f.options;

    // The alternate flag is already treated by LowerHex as being special-
    // it denotes whether to prefix with 0x. We use it to work out whether
    // or not to zero extend, and then unconditionally set it to get the
    // prefix.
    if f.options.get_alternate() {
        f.options.sign_aware_zero_pad(true);

        if f.options.get_width().is_none() {
            f.options.width(Some((usize::BITS / 4) as u16 + 2));
        }
    }
    f.options.alternate(true);

    let ret = LowerHex::fmt(&ptr_addr, f);

    f.options = old_options;

    ret
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PointeeSized> Pointer for *mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&(*self as *const T), f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PointeeSized> Pointer for &T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&(*self as *const T), f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PointeeSized> Pointer for &mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(&(&**self as *const T), f)
    }
}

// Implementation of Display/Debug for various core types

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PointeeSized> Debug for *const T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(self, f)
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: PointeeSized> Debug for *mut T {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Pointer::fmt(self, f)
    }
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
}

macro_rules! tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        maybe_tuple_doc! {
            $($name)+ @
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($name:Debug),+> Debug for ($($name,)+) {
                #[allow(non_snake_case, unused_assignments)]
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    let mut builder = f.debug_tuple("");
                    let ($(ref $name,)+) = *self;
                    $(
                        builder.field(&$name);
                    )+

                    builder.finish()
                }
            }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! maybe_tuple_doc {
    ($a:ident @ #[$meta:meta] $item:item) => {
        #[doc(fake_variadic)]
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        #[$meta]
        $item
    };
    ($a:ident $($rest_a:ident)+ @ #[$meta:meta] $item:item) => {
        #[doc(hidden)]
        #[$meta]
        $item
    };
}

tuple! { E, D, C, B, A, Z, Y, X, W, V, U, T, }

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Debug> Debug for [T] {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl Debug for () {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad("()")
    }
}
#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized> Debug for PhantomData<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PhantomData<{}>", crate::any::type_name::<T>())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: Copy + Debug> Debug for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Cell").field("value", &self.get()).finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + Debug> Debug for RefCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut d = f.debug_struct("RefCell");
        match self.try_borrow() {
            Ok(borrow) => d.field("value", &borrow),
            Err(_) => d.field("value", &format_args!("<borrowed>")),
        };
        d.finish()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + Debug> Debug for Ref<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&**self, f)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ?Sized + Debug> Debug for RefMut<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&*(self.deref()), f)
    }
}

#[stable(feature = "core_impl_debug", since = "1.9.0")]
impl<T: ?Sized> Debug for UnsafeCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("UnsafeCell").finish_non_exhaustive()
    }
}

#[unstable(feature = "sync_unsafe_cell", issue = "95439")]
impl<T: ?Sized> Debug for SyncUnsafeCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("SyncUnsafeCell").finish_non_exhaustive()
    }
}

// If you expected tests to be here, look instead at coretests/tests/fmt/;
// it's a lot easier than creating all of the rt::Piece structures here.
// There are also tests in alloctests/tests/fmt.rs, for those that need allocations.
```

---
*Generated by AST tracing system*
