// Generated macro for impl_2569 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2569 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2569"}
// Dependencies: {}
impl fmt :: Display for AddrO32 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AddrO32 :: Base { addr , offset } => { let addr = reg_name (* * addr) ; write ! (f , "{addr}, {offset}") } } } }
};
}
