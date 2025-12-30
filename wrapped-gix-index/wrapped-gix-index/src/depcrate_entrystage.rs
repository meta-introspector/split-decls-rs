// Generated macro for Stage (enum)
macro_rules! Depcrate_entryStage {
() => {
// Module: crate::entry
// Provides: {"Stage"}
// Dependencies: {}
# [doc = " The stage of an entry."] # [derive (Default , Copy , Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash)] pub enum Stage { # [doc = " This is the default, and most entries are in this stage."] # [default] Unconflicted = 0 , # [doc = " The entry is the common base between 'our' change and 'their' change, for comparison."] Base = 1 , # [doc = " The entry represents our change."] Ours = 2 , # [doc = " The entry represents their change."] Theirs = 3 , }
};
}
