// Generated macro for impl_476 (impl)
macro_rules! Depcrate_arbitrary__std_pathimpl_476 {
() => {
// Module: crate::arbitrary::_std::path
// Provides: {"impl_476"}
// Dependencies: {}
impl Arbitrary for PathParamsOutput { type Parameters = PathParams ; type Strategy = SMapped < (bool , Vec < String >) , Self > ; fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { static_map ((any :: < bool > () , any_with :: < Vec < String > > ((args . components () , args . component_regex () ,)) ,) , | (is_absolute , components) | Self { is_absolute , components , } ,) } }
};
}
