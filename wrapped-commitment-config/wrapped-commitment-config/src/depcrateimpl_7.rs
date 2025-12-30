// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl FromStr for CommitmentLevel { type Err = ParseCommitmentLevelError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s { "processed" => Ok (CommitmentLevel :: Processed) , "confirmed" => Ok (CommitmentLevel :: Confirmed) , "finalized" => Ok (CommitmentLevel :: Finalized) , _ => Err (ParseCommitmentLevelError :: Invalid) , } } }
};
}
