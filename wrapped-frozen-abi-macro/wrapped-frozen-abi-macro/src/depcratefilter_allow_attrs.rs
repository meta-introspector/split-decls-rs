// Generated macro for filter_allow_attrs (function)
macro_rules! Depcratefilter_allow_attrs {
() => {
// Module: crate
// Provides: {"filter_allow_attrs"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] fn filter_allow_attrs (attrs : & mut Vec < Attribute >) { attrs . retain (| attr | { let ss = & attr . path () . segments . first () . unwrap () . ident . to_string () ; ss . starts_with ("allow") }) ; }
};
}
