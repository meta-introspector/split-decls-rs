// Generated macro for impl_678 (impl)
macro_rules! Depcrate_mergeimpl_678 {
() => {
// Module: crate::merge
// Provides: {"impl_678"}
// Dependencies: {}
impl std :: fmt :: Debug for MergeFileResult { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut ds = f . debug_struct ("MergeFileResult") ; if let Some (path) = & self . path () { ds . field ("path" , path) ; } ds . field ("automergeable" , & self . is_automergeable ()) ; ds . field ("mode" , & self . mode ()) ; ds . finish () } }
};
}
