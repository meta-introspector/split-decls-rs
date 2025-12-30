// Generated macro for impl_2252 (impl)
macro_rules! Depcrate_isa_riscv64impl_2252 {
() => {
// Module: crate::isa::riscv64
// Provides: {"impl_2252"}
// Dependencies: {}
impl fmt :: Display for Riscv64Backend { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("MachBackend") . field ("name" , & self . name ()) . field ("triple" , & self . triple ()) . field ("flags" , & format ! ("{}" , self . flags ())) . finish () } }
};
}
