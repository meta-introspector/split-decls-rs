// Generated macro for Semantics (struct)
macro_rules! Depcrate_semanticsSemantics {
() => {
// Module: crate::semantics
// Provides: {"Semantics"}
// Dependencies: {}
# [doc = " Primary API to get semantic information, like types, from syntax trees."] pub struct Semantics < 'db , DB : ? Sized > { pub db : & 'db DB , imp : SemanticsImpl < 'db > , }
};
}
