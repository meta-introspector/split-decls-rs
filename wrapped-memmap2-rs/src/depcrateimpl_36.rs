// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl fmt :: Debug for MmapRaw { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("MmapRaw") . field ("ptr" , & self . as_ptr ()) . field ("len" , & self . len ()) . finish () } }
};
}
