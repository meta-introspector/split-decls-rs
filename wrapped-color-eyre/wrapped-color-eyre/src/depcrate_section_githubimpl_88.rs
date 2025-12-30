// Generated macro for impl_88 (impl)
macro_rules! Depcrate_section_githubimpl_88 {
() => {
// Module: crate::section::github
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a , T > MetadataSection < T > where T : IntoIterator < Item = & 'a (String , Display < 'a >) > , { # [allow (clippy :: inherent_to_string , clippy :: wrong_self_convention)] fn to_string (self) -> String { use std :: fmt :: Write ; let mut out = String :: new () ; let f = & mut out ; writeln ! (f , "|key|value|") . expect ("writing to a string doesn't panic") ; writeln ! (f , "|--|--|") . expect ("writing to a string doesn't panic") ; for (key , value) in self . metadata { writeln ! (f , "|**{key}**|{value}|") . expect ("writing to a string doesn't panic") ; } out } }
};
}
