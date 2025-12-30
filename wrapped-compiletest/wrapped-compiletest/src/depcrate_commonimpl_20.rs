// Generated macro for impl_20 (impl)
macro_rules! Depcrate_commonimpl_20 {
() => {
// Module: crate::common
// Provides: {"impl_20"}
// Dependencies: {}
impl TestMode { pub fn aux_dir_disambiguator (self) -> & 'static str { match self { TestMode :: Pretty => ".pretty" , _ => "" , } } pub fn output_dir_disambiguator (self) -> & 'static str { match self { TestMode :: CoverageMap | TestMode :: CoverageRun => self . to_str () , _ => "" , } } }
};
}
