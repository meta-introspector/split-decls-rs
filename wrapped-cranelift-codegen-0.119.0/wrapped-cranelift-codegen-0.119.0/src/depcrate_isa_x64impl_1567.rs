// Generated macro for impl_1567 (impl)
macro_rules! Depcrate_isa_x64impl_1567 {
() => {
// Module: crate::isa::x64
// Provides: {"impl_1567"}
// Dependencies: {}
impl fmt :: Display for X64Backend { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MachBackend") . field ("name" , & self . name ()) . field ("triple" , & self . triple ()) . field ("flags" , & format ! ("{}" , self . flags ())) . finish () } }
};
}
