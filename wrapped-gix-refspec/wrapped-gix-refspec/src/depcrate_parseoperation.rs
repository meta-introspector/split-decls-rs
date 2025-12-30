// Generated macro for Operation (enum)
macro_rules! Depcrate_parseOperation {
() => {
// Module: crate::parse
// Provides: {"Operation"}
// Dependencies: {}
# [doc = " Define how the parsed refspec should be used."] # [derive (PartialOrd , Ord , PartialEq , Eq , Copy , Clone , Hash , Debug)] pub enum Operation { # [doc = " The `src` side is local and the `dst` side is remote."] Push , # [doc = " The `src` side is remote and the `dst` side is local."] Fetch , }
};
}
