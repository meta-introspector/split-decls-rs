// Generated macro for impl_2067 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2067 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2067"}
// Dependencies: {}
impl AMO { pub (crate) fn to_static_str (self) -> & 'static str { match self { AMO :: Relax => "" , AMO :: Release => ".rl" , AMO :: Acquire => ".aq" , AMO :: SeqCst => ".aqrl" , } } pub (crate) fn as_u32 (self) -> u32 { self as u32 } }
};
}
