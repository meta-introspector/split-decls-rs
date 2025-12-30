// Generated macro for impl_2574 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2574 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2574"}
// Dependencies: {}
impl fmt :: Display for AddrZ { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { AddrZ :: Base { addr , offset } => { let addr = reg_name (* * addr) ; write ! (f , "{addr}, {offset}") } } } }
};
}
