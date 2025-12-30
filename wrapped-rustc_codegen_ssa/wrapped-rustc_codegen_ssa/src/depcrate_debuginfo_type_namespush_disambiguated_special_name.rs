// Generated macro for push_disambiguated_special_name (function)
macro_rules! Depcrate_debuginfo_type_namespush_disambiguated_special_name {
() => {
// Module: crate::debuginfo::type_names
// Provides: {"push_disambiguated_special_name"}
// Dependencies: {}
fn push_disambiguated_special_name (label : & str , disambiguator : u32 , cpp_like_debuginfo : bool , output : & mut String ,) { if cpp_like_debuginfo { write ! (output , "{label}${disambiguator}") . unwrap () ; } else { write ! (output , "{{{label}#{disambiguator}}}") . unwrap () ; } }
};
}
