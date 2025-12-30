// Generated macro for TermTarget (enum)
macro_rules! Depcrate_termTermTarget {
() => {
// Module: crate::term
// Provides: {"TermTarget"}
// Dependencies: {}
# [doc = " Where the term is writing."] # [derive (Debug , Clone)] pub enum TermTarget { Stdout , Stderr , # [cfg (unix)] ReadWritePair (ReadWritePair) , }
};
}
