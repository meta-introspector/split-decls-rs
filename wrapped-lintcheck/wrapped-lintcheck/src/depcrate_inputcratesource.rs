// Generated macro for CrateSource (enum)
macro_rules! Depcrate_inputCrateSource {
() => {
// Module: crate::input
// Provides: {"CrateSource"}
// Dependencies: {}
# [derive (Debug , Deserialize , Eq , Hash , PartialEq , Ord , PartialOrd)] pub enum CrateSource { CratesIo { version : String } , Git { url : String , commit : String } , Path { path : PathBuf } , }
};
}
