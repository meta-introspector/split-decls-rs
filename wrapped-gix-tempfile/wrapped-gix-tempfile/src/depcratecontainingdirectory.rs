// Generated macro for ContainingDirectory (enum)
macro_rules! DepcrateContainingDirectory {
() => {
// Module: crate
// Provides: {"ContainingDirectory"}
// Dependencies: {}
# [doc = " A type expressing the ways we can deal with directories containing a tempfile."] # [derive (Debug , Clone , Copy , Ord , PartialOrd , Eq , PartialEq)] pub enum ContainingDirectory { # [doc = " Assume the directory for the tempfile exists and cause failure if it doesn't"] Exists , # [doc = " Create the directory recursively with the given number of retries in a way that is somewhat race resistant"] # [doc = " depending on the amount of retries."] CreateAllRaceProof (create_dir :: Retries) , }
};
}
