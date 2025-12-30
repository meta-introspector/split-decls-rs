// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl FromStr for CommitmentConfig { type Err = ParseCommitmentLevelError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { CommitmentLevel :: from_str (s) . map (| commitment | Self { commitment }) } }
};
}
