// Generated macro for Other (struct)
macro_rules! Depcrate_extensions_otherOther {
() => {
// Module: crate::extensions::other
// Provides: {"Other"}
// Dependencies: {}
# [doc = " A list of [`Other Use Extensions`] as defined in [`Unicode Locale"] # [doc = " Identifier`] specification."] # [doc = ""] # [doc = " Those extensions are treated as a pass-through, and no Unicode related"] # [doc = " behavior depends on them."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::other::Other;"] # [doc = " use icu::locale::subtags::Subtag;"] # [doc = ""] # [doc = " let subtag1: Subtag = \"foo\".parse().expect(\"Failed to parse a Subtag.\");"] # [doc = " let subtag2: Subtag = \"bar\".parse().expect(\"Failed to parse a Subtag.\");"] # [doc = ""] # [doc = " let other = Other::from_vec_unchecked(b'a', vec![subtag1, subtag2]);"] # [doc = " assert_eq!(&other.to_string(), \"a-foo-bar\");"] # [doc = " ```"] # [doc = ""] # [doc = " [`Other Use Extensions`]: https://unicode.org/reports/tr35/#other_extensions"] # [doc = " [`Unicode Locale Identifier`]: https://unicode.org/reports/tr35/#Unicode_locale_identifier"] # [derive (Clone , PartialEq , Eq , Debug , Default , Hash , PartialOrd , Ord)] pub struct Other { ext : u8 , keys : ShortBoxSlice < Subtag > , }
};
}
