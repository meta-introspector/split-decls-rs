// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl fmt :: Debug for MmapMut { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("MmapMut") . field ("ptr" , & self . as_ptr ()) . field ("len" , & self . len ()) . finish () } }
};
}
