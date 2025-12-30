// Generated macro for impl_544 (impl)
macro_rules! Depcrate_diffimpl_544 {
() => {
// Module: crate::diff
// Provides: {"impl_544"}
// Dependencies: {}
impl < 'a > std :: fmt :: Debug for DiffLine < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("DiffLine") ; if let Some (old_lineno) = & self . old_lineno () { ds . field ("old_lineno" , old_lineno) ; } if let Some (new_lineno) = & self . new_lineno () { ds . field ("new_lineno" , new_lineno) ; } ds . field ("num_lines" , & self . num_lines ()) . field ("content_offset" , & self . content_offset ()) . field ("content" , & self . content ()) . field ("origin" , & self . origin ()) . finish () } }
};
}
