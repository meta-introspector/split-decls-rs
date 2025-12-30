// Generated macro for impl_2568 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_argsimpl_2568 {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"impl_2568"}
// Dependencies: {}
impl From < AddrO32 > for pulley_interpreter :: AddrO32 { fn from (addr : AddrO32) -> Self { match addr { AddrO32 :: Base { addr , offset } => Self { addr : addr . into () , offset , } , } } }
};
}
