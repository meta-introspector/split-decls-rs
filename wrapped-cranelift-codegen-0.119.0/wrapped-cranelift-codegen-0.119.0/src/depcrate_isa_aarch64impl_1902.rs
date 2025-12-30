// Generated macro for impl_1902 (impl)
macro_rules! Depcrate_isa_aarch64impl_1902 {
() => {
// Module: crate::isa::aarch64
// Provides: {"impl_1902"}
// Dependencies: {}
impl fmt :: Display for AArch64Backend { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MachBackend") . field ("name" , & self . name ()) . field ("triple" , & self . triple ()) . field ("flags" , & format ! ("{}" , self . flags ())) . finish () } }
};
}
