// Generated macro for diff (function)
macro_rules! Depcrate_myersdiff {
() => {
// Module: crate::myers
// Provides: {"diff"}
// Dependencies: {}
pub fn diff (before : & [Token] , after : & [Token] , removed : & mut [bool] , added : & mut [bool] , minimal : bool ,) { let (before , after) = preprocess :: preprocess (before , after , removed , added) ; Myers :: new (before . tokens . len () , after . tokens . len ()) . run (FileSlice :: new (& before , removed) , FileSlice :: new (& after , added) , minimal ,) ; }
};
}
