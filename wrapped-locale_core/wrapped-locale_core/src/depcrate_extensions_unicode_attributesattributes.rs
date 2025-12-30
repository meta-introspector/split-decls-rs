// Generated macro for Attributes (struct)
macro_rules! Depcrate_extensions_unicode_attributesAttributes {
() => {
// Module: crate::extensions::unicode::attributes
// Provides: {"Attributes"}
// Dependencies: {}
# [doc = " A set of [`Attribute`] elements as defined in [`Unicode Extension Attributes`]."] # [doc = ""] # [doc = " [`Unicode Extension Attributes`]: https://unicode.org/reports/tr35/tr35.html#u_Extension"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::unicode::{Attribute, Attributes};"] # [doc = ""] # [doc = " let attribute1: Attribute ="] # [doc = "     \"foobar\".parse().expect(\"Failed to parse a variant subtag.\");"] # [doc = ""] # [doc = " let attribute2: Attribute = \"testing\""] # [doc = "     .parse()"] # [doc = "     .expect(\"Failed to parse a variant subtag.\");"] # [doc = " let mut v = vec![attribute1, attribute2];"] # [doc = " v.sort();"] # [doc = " v.dedup();"] # [doc = ""] # [doc = " let attributes: Attributes = Attributes::from_vec_unchecked(v);"] # [doc = " assert_eq!(attributes.to_string(), \"foobar-testing\");"] # [doc = " ```"] # [derive (Default , Debug , PartialEq , Eq , Clone , Hash , PartialOrd , Ord)] pub struct Attributes (ShortBoxSlice < Attribute >) ;
};
}
