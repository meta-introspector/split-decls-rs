// Generated macro for impl_533 (impl)
macro_rules! Depcrate_diffimpl_533 {
() => {
// Module: crate::diff
// Provides: {"impl_533"}
// Dependencies: {}
impl < 'a > std :: fmt :: Debug for DiffFile < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("DiffFile") ; ds . field ("id" , & self . id ()) ; if let Some (path_bytes) = & self . path_bytes () { ds . field ("path_bytes" , path_bytes) ; } if let Some (path) = & self . path () { ds . field ("path" , path) ; } ds . field ("size" , & self . size ()) . finish () } }
};
}
