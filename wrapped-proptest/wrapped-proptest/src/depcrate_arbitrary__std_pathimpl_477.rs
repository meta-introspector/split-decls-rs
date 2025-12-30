// Generated macro for impl_477 (impl)
macro_rules! Depcrate_arbitrary__std_pathimpl_477 {
() => {
// Module: crate::arbitrary::_std::path
// Provides: {"impl_477"}
// Dependencies: {}
# [doc = " This implementation accepts as its argument a [`PathParams`] struct. It generates either a"] # [doc = " relative or an absolute path with equal probability."] # [doc = ""] # [doc = " Currently, this implementation does not generate:"] # [doc = ""] # [doc = " * Paths that are not valid UTF-8 (this is unlikely to change)"] # [doc = " * Paths with a [`PrefixComponent`](std::path::PrefixComponent) on Windows, e.g. `C:\\` (this may"] # [doc = "   change in the future)"] impl Arbitrary for PathBuf { type Parameters = PathParams ; type Strategy = SMapped < PathParamsOutput , Self > ; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { static_map (any_with :: < PathParamsOutput > (args) , | PathParamsOutput { is_absolute , components , } | { let mut out = PathBuf :: new () ; if is_absolute { out . push (& MAIN_SEPARATOR . to_string ()) ; } for component in components { let component = component . chars () . filter (| & c | ! std :: path :: is_separator (c)) . collect :: < String > () ; out . push (& component) ; } out } ,) } }
};
}
