use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, T> Symbol<&'a T>
where
    T: AsRef<[u8]> + ?Sized,
{
    /// Parse a mangled symbol from input and return it and the trailing tail of
    /// bytes that come after the symbol, with the default options.
    ///
    /// While `Symbol::new` will return an error if there is unexpected trailing
    /// bytes, `with_tail` simply returns the trailing bytes along with the
    /// parsed symbol.
    ///
    /// ```
    /// use cpp_demangle::BorrowedSymbol;
    ///
    /// let mangled = b"_ZN5space3fooEibc and some trailing junk";
    ///
    /// let (sym, tail) = BorrowedSymbol::with_tail(&mangled[..])
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// assert_eq!(tail, b" and some trailing junk");
    ///
    /// let demangled = sym.demangle().unwrap();
    /// assert_eq!(demangled, "space::foo(int, bool, char)");
    /// ```
    #[inline]
    pub fn with_tail(input: &'a T) -> Result<(BorrowedSymbol<'a>, &'a [u8])> {
        Self::with_tail_and_options(input, &Default::default())
    }
    /// Parse a mangled symbol from input and return it and the trailing tail of
    /// bytes that come after the symbol.
    ///
    /// While `Symbol::new_with_options` will return an error if there is
    /// unexpected trailing bytes, `with_tail_and_options` simply returns the
    /// trailing bytes along with the parsed symbol.
    ///
    /// ```
    /// use cpp_demangle::{BorrowedSymbol, ParseOptions};
    ///
    /// let mangled = b"_ZN5space3fooEibc and some trailing junk";
    ///
    /// let parse_options = ParseOptions::default()
    ///     .recursion_limit(1024);
    ///
    /// let (sym, tail) = BorrowedSymbol::with_tail_and_options(&mangled[..], &parse_options)
    ///     .expect("Could not parse mangled symbol!");
    ///
    /// assert_eq!(tail, b" and some trailing junk");
    ///
    /// let demangled = sym.demangle().unwrap();
    /// assert_eq!(demangled, "space::foo(int, bool, char)");
    /// ```
    pub fn with_tail_and_options(
        input: &'a T,
        options: &ParseOptions,
    ) -> Result<(BorrowedSymbol<'a>, &'a [u8])> {
        let mut substitutions = subs::SubstitutionTable::new();
        let ctx = ParseContext::new(*options);
        let idx_str = IndexStr::new(input.as_ref());
        let (parsed, tail) = ast::MangledName::parse(&ctx, &mut substitutions, idx_str)?;
        debug_assert!(ctx.recursion_level() == 0);
        let symbol = Symbol {
            raw: input.as_ref(),
            substitutions: substitutions,
            parsed: parsed,
        };
        log!(
            "Successfully parsed '{}' as

AST = {:#?}

substitutions = {:#?}",
            String::from_utf8_lossy(symbol.raw),
            symbol.parsed,
            symbol.substitutions
        );
        Ok((symbol, tail.into()))
    }
}
