// Generated macro for impl_2220 (impl)
macro_rules! Depcrate_isa_riscv64_instimpl_2220 {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"impl_2220"}
// Dependencies: {}
impl Display for CondBrTarget { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { match self { CondBrTarget :: Label (l) => write ! (f , "{}" , l . to_string ()) , CondBrTarget :: Fallthrough => write ! (f , "0") , } } }
};
}
