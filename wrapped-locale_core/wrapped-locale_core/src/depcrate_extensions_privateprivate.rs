// Generated macro for Private (struct)
macro_rules! Depcrate_extensions_privatePrivate {
() => {
// Module: crate::extensions::private
// Provides: {"Private"}
// Dependencies: {}
# [doc = " A list of [`Private Use Extensions`] as defined in [`Unicode Locale"] # [doc = " Identifier`] specification."] # [doc = ""] # [doc = " Those extensions are treated as a pass-through, and no Unicode related"] # [doc = " behavior depends on them."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::private::{Private, Subtag};"] # [doc = ""] # [doc = " let subtag1: Subtag = \"foo\".parse().expect(\"Failed to parse a Subtag.\");"] # [doc = " let subtag2: Subtag = \"bar\".parse().expect(\"Failed to parse a Subtag.\");"] # [doc = ""] # [doc = " let private = Private::from_vec_unchecked(vec![subtag1, subtag2]);"] # [doc = " assert_eq!(&private.to_string(), \"x-foo-bar\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`Private Use Extensions`]: https://unicode.org/reports/tr35/#pu_extensions"] # [doc = " [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier"] # [derive (Clone , PartialEq , Eq , Debug , Default , Hash , PartialOrd , Ord)] pub struct Private (ShortBoxSlice < Subtag >) ;
};
}
