// Generated macro for SynValue (enum)
macro_rules! Depcrate_syn_interpreterSynValue {
() => {
// Module: crate::syn_interpreter
// Provides: {"SynValue"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum SynValue { Path (PathBuf) , String (String) , Bool (bool) , Result (Box < SynValue >) , Unit , }
};
}
