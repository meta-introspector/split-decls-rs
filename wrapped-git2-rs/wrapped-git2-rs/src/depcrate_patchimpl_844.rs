// Generated macro for impl_844 (impl)
macro_rules! Depcrate_patchimpl_844 {
() => {
// Module: crate::patch
// Provides: {"impl_844"}
// Dependencies: {}
impl < 'buffers > std :: fmt :: Debug for Patch < 'buffers > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Patch") ; ds . field ("delta" , & self . delta ()) . field ("num_hunks" , & self . num_hunks ()) ; if let Ok (line_stats) = & self . line_stats () { ds . field ("line_stats" , line_stats) ; } ds . finish () } }
};
}
