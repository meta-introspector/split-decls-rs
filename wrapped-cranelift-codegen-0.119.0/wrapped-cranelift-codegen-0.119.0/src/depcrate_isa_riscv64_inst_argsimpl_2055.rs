// Generated macro for impl_2055 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2055 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2055"}
// Dependencies: {}
impl TryFrom < Type > for FpuOPWidth { type Error = & 'static str ; fn try_from (value : Type) -> std :: result :: Result < Self , Self :: Error > { match value { F16 => Ok (FpuOPWidth :: H) , F32 => Ok (FpuOPWidth :: S) , F64 => Ok (FpuOPWidth :: D) , F128 => Ok (FpuOPWidth :: Q) , _ => Err ("Invalid type for FpuOPWidth") , } } }
};
}
