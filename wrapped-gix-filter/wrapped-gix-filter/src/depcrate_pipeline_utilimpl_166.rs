// Generated macro for impl_166 (impl)
macro_rules! Depcrate_pipeline_utilimpl_166 {
() => {
// Module: crate::pipeline::util
// Provides: {"impl_166"}
// Dependencies: {}
impl CrlfRoundTripCheck { pub (crate) fn to_eol_roundtrip_check (self , rela_path : & Path) -> Option < eol :: convert_to_git :: RoundTripCheck < '_ > > { match self { CrlfRoundTripCheck :: Fail => Some (eol :: convert_to_git :: RoundTripCheck :: Fail { rela_path }) , CrlfRoundTripCheck :: Warn => Some (eol :: convert_to_git :: RoundTripCheck :: Warn { rela_path }) , CrlfRoundTripCheck :: Skip => None , } } }
};
}
