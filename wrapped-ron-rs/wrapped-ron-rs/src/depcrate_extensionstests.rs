// Generated macro for tests (module)
macro_rules! Depcrate_extensionstests {
() => {
// Module: crate::extensions
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Extensions ; fn roundtrip_extensions (ext : Extensions) { let ron = crate :: to_string (& ext) . unwrap () ; let ext2 : Extensions = crate :: from_str (& ron) . unwrap () ; assert_eq ! (ext , ext2) ; } # [test] fn test_extension_serde () { for bits in Extensions :: empty () . bits () ..= Extensions :: all () . bits () { let extensions = Extensions :: from_bits_retain (bits) ; roundtrip_extensions (extensions) ; } } }
};
}
