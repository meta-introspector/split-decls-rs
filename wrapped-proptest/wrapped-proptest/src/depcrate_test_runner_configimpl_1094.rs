// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_test_runner_configimpl_1094 {
() => {
// Module: crate::test_runner::config
// Provides: {"impl_1094"}
// Dependencies: {}
impl str :: FromStr for RngSeed { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { s . parse :: < u64 > () . map (RngSeed :: Fixed) . map_err (| _ | ()) } }
};
}
