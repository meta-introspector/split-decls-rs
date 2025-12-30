// Generated macro for Fields (struct)
macro_rules! Depcrate_extensions_transform_fieldsFields {
() => {
// Module: crate::extensions::transform::fields
// Provides: {"Fields"}
// Dependencies: {}
# [doc = " A list of [`Key`]-[`Value`] pairs representing functional information"] # [doc = " about content transformations."] # [doc = ""] # [doc = " Here are examples of fields used in Unicode:"] # [doc = " - `s0`, `d0` - Transform source/destination"] # [doc = " - `t0` - Machine Translation"] # [doc = " - `h0` - Hybrid Locale Identifiers"] # [doc = ""] # [doc = " You can find the full list in [`Unicode BCP 47 T Extension`] section of LDML."] # [doc = ""] # [doc = " [`Unicode BCP 47 T Extension`]: https://unicode.org/reports/tr35/tr35.html#BCP47_T_Extension"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::locale::extensions::transform::{key, Fields, Value};"] # [doc = ""] # [doc = " let value = \"hybrid\".parse::<Value>().expect(\"Failed to parse a Value.\");"] # [doc = " let fields = [(key!(\"h0\"), value)].into_iter().collect::<Fields>();"] # [doc = ""] # [doc = " assert_eq!(&fields.to_string(), \"h0-hybrid\");"] # [doc = " ```"] # [derive (Clone , PartialEq , Eq , Debug , Default , Hash , PartialOrd , Ord)] pub struct Fields (Inner) ;
};
}
