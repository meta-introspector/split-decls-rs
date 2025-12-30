// Generated macro for impl_2558 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2558 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2558"}
// Dependencies: {}
impl core :: fmt :: Display for Amode { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { Amode :: SpOffset { offset } => { if * offset >= 0 { write ! (f , "sp+{offset}") } else { write ! (f , "sp{offset}") } } Amode :: RegOffset { base , offset } => { let name = reg_name (* * base) ; if * offset >= 0 { write ! (f , "{name}+{offset}") } else { write ! (f , "{name}{offset}") } } Amode :: Stack { amode } => core :: fmt :: Debug :: fmt (amode , f) , } } }
};
}
