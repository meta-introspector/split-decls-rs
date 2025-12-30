// Generated macro for impl_152 (impl)
macro_rules! Depcrate_internalimpl_152 {
() => {
// Module: crate::internal
// Provides: {"impl_152"}
// Dependencies: {}
impl fmt :: Debug for Bag { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Bag") . field ("deferreds" , & & self . deferreds [.. self . len]) . finish () } }
};
}
