// Generated macro for impl_530 (impl)
macro_rules! Depcrate_diffimpl_530 {
() => {
// Module: crate::diff
// Provides: {"impl_530"}
// Dependencies: {}
impl < 'a > std :: fmt :: Debug for DiffDelta < 'a > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_struct ("DiffDelta") . field ("nfiles" , & self . nfiles ()) . field ("status" , & self . status ()) . field ("old_file" , & self . old_file ()) . field ("new_file" , & self . new_file ()) . finish () } }
};
}
