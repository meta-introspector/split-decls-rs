use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Literal {
    fn _new(inner: imp::Literal) -> Self {
        Literal {
            inner,
            _marker: MARKER,
        }
    }
    fn _new_fallback(inner: fallback::Literal) -> Self {
        Literal {
            inner: imp::Literal::from(inner),
            _marker: MARKER,
        }
    }
    suffixed_int_literals! {
        u8_suffixed => u8, u16_suffixed => u16, u32_suffixed => u32, u64_suffixed => u64,
        u128_suffixed => u128, usize_suffixed => usize, i8_suffixed => i8, i16_suffixed
        => i16, i32_suffixed => i32, i64_suffixed => i64, i128_suffixed => i128,
        isize_suffixed => isize,
    }
    unsuffixed_int_literals! {
        u8_unsuffixed => u8, u16_unsuffixed => u16, u32_unsuffixed => u32, u64_unsuffixed
        => u64, u128_unsuffixed => u128, usize_unsuffixed => usize, i8_unsuffixed => i8,
        i16_unsuffixed => i16, i32_unsuffixed => i32, i64_unsuffixed => i64,
        i128_unsuffixed => i128, isize_unsuffixed => isize,
    }
    /// Creates a new unsuffixed floating-point literal.
    ///
    /// This constructor is similar to those like `Literal::i8_unsuffixed` where
    /// the float's value is emitted directly into the token but no suffix is
    /// used, so it may be inferred to be a `f64` later in the compiler.
    /// Literals created from negative numbers may not survive round-trips
    /// through `TokenStream` or strings and may be broken into two tokens (`-`
    /// and positive literal).
    ///
    /// # Panics
    ///
    /// This function requires that the specified float is finite, for example
    /// if it is infinity or NaN this function will panic.
    pub fn f64_unsuffixed(f: f64) -> Literal {
        assert!(f.is_finite());
        Literal::_new(imp::Literal::f64_unsuffixed(f))
    }
    /// Creates a new suffixed floating-point literal.
    ///
    /// This constructor will create a literal like `1.0f64` where the value
    /// specified is the preceding part of the token and `f64` is the suffix of
    /// the token. This token will always be inferred to be an `f64` in the
    /// compiler. Literals created from negative numbers may not survive
    /// round-trips through `TokenStream` or strings and may be broken into two
    /// tokens (`-` and positive literal).
    ///
    /// # Panics
    ///
    /// This function requires that the specified float is finite, for example
    /// if it is infinity or NaN this function will panic.
    pub fn f64_suffixed(f: f64) -> Literal {
        assert!(f.is_finite());
        Literal::_new(imp::Literal::f64_suffixed(f))
    }
    /// Creates a new unsuffixed floating-point literal.
    ///
    /// This constructor is similar to those like `Literal::i8_unsuffixed` where
    /// the float's value is emitted directly into the token but no suffix is
    /// used, so it may be inferred to be a `f64` later in the compiler.
    /// Literals created from negative numbers may not survive round-trips
    /// through `TokenStream` or strings and may be broken into two tokens (`-`
    /// and positive literal).
    ///
    /// # Panics
    ///
    /// This function requires that the specified float is finite, for example
    /// if it is infinity or NaN this function will panic.
    pub fn f32_unsuffixed(f: f32) -> Literal {
        assert!(f.is_finite());
        Literal::_new(imp::Literal::f32_unsuffixed(f))
    }
    /// Creates a new suffixed floating-point literal.
    ///
    /// This constructor will create a literal like `1.0f32` where the value
    /// specified is the preceding part of the token and `f32` is the suffix of
    /// the token. This token will always be inferred to be an `f32` in the
    /// compiler. Literals created from negative numbers may not survive
    /// round-trips through `TokenStream` or strings and may be broken into two
    /// tokens (`-` and positive literal).
    ///
    /// # Panics
    ///
    /// This function requires that the specified float is finite, for example
    /// if it is infinity or NaN this function will panic.
    pub fn f32_suffixed(f: f32) -> Literal {
        assert!(f.is_finite());
        Literal::_new(imp::Literal::f32_suffixed(f))
    }
    /// String literal.
    pub fn string(string: &str) -> Literal {
        Literal::_new(imp::Literal::string(string))
    }
    /// Character literal.
    pub fn character(ch: char) -> Literal {
        Literal::_new(imp::Literal::character(ch))
    }
    /// Byte character literal.
    pub fn byte_character(byte: u8) -> Literal {
        Literal::_new(imp::Literal::byte_character(byte))
    }
    /// Byte string literal.
    pub fn byte_string(bytes: &[u8]) -> Literal {
        Literal::_new(imp::Literal::byte_string(bytes))
    }
    /// C string literal.
    pub fn c_string(string: &CStr) -> Literal {
        Literal::_new(imp::Literal::c_string(string))
    }
    /// Returns the span encompassing this literal.
    pub fn span(&self) -> Span {
        Span::_new(self.inner.span())
    }
    /// Configures the span associated for this literal.
    pub fn set_span(&mut self, span: Span) {
        self.inner.set_span(span.inner);
    }
    /// Returns a `Span` that is a subset of `self.span()` containing only
    /// the source bytes in range `range`. Returns `None` if the would-be
    /// trimmed span is outside the bounds of `self`.
    ///
    /// Warning: the underlying [`proc_macro::Literal::subspan`] method is
    /// nightly-only. When called from within a procedural macro not using a
    /// nightly compiler, this method will always return `None`.
    pub fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {
        self.inner.subspan(range).map(Span::_new)
    }
    /// Returns the unescaped string value if this is a string literal.
    #[cfg(procmacro2_semver_exempt)]
    pub fn str_value(&self) -> Result<String, ConversionErrorKind> {
        let repr = self.to_string();
        if repr.starts_with('"') && repr[1..].ends_with('"') {
            let quoted = &repr[1..repr.len() - 1];
            let mut value = String::with_capacity(quoted.len());
            let mut error = None;
            rustc_literal_escaper::unescape_str(quoted, |_range, res| match res {
                Ok(ch) => value.push(ch),
                Err(err) => {
                    if err.is_fatal() {
                        error = Some(ConversionErrorKind::FailedToUnescape(err));
                    }
                }
            });
            return match error {
                Some(error) => Err(error),
                None => Ok(value),
            };
        }
        if repr.starts_with('r') {
            if let Some(raw) = get_raw(&repr[1..]) {
                return Ok(raw.to_owned());
            }
        }
        Err(ConversionErrorKind::InvalidLiteralKind)
    }
    /// Returns the unescaped string value (including nul terminator) if this is
    /// a c-string literal.
    #[cfg(procmacro2_semver_exempt)]
    pub fn cstr_value(&self) -> Result<Vec<u8>, ConversionErrorKind> {
        let repr = self.to_string();
        if repr.starts_with("c\"") && repr[2..].ends_with('"') {
            let quoted = &repr[2..repr.len() - 1];
            let mut value = Vec::with_capacity(quoted.len());
            let mut error = None;
            rustc_literal_escaper::unescape_c_str(quoted, |_range, res| match res {
                Ok(MixedUnit::Char(ch)) => {
                    value.extend_from_slice(ch.get().encode_utf8(&mut [0; 4]).as_bytes());
                }
                Ok(MixedUnit::HighByte(byte)) => value.push(byte.get()),
                Err(err) => {
                    if err.is_fatal() {
                        error = Some(ConversionErrorKind::FailedToUnescape(err));
                    }
                }
            });
            return match error {
                Some(error) => Err(error),
                None => {
                    value.push(b'\0');
                    Ok(value)
                }
            };
        }
        if repr.starts_with("cr") {
            if let Some(raw) = get_raw(&repr[2..]) {
                let mut value = Vec::with_capacity(raw.len() + 1);
                value.extend_from_slice(raw.as_bytes());
                value.push(b'\0');
                return Ok(value);
            }
        }
        Err(ConversionErrorKind::InvalidLiteralKind)
    }
    /// Returns the unescaped string value if this is a byte string literal.
    #[cfg(procmacro2_semver_exempt)]
    pub fn byte_str_value(&self) -> Result<Vec<u8>, ConversionErrorKind> {
        let repr = self.to_string();
        if repr.starts_with("b\"") && repr[2..].ends_with('"') {
            let quoted = &repr[2..repr.len() - 1];
            let mut value = Vec::with_capacity(quoted.len());
            let mut error = None;
            rustc_literal_escaper::unescape_byte_str(quoted, |_range, res| match res {
                Ok(byte) => value.push(byte),
                Err(err) => {
                    if err.is_fatal() {
                        error = Some(ConversionErrorKind::FailedToUnescape(err));
                    }
                }
            });
            return match error {
                Some(error) => Err(error),
                None => Ok(value),
            };
        }
        if repr.starts_with("br") {
            if let Some(raw) = get_raw(&repr[2..]) {
                return Ok(raw.as_bytes().to_owned());
            }
        }
        Err(ConversionErrorKind::InvalidLiteralKind)
    }
    #[doc(hidden)]
    pub unsafe fn from_str_unchecked(repr: &str) -> Self {
        Literal::_new(unsafe { imp::Literal::from_str_unchecked(repr) })
    }
}
