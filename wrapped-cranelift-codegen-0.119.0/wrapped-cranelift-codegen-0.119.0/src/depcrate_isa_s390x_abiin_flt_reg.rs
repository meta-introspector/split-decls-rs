// Generated macro for in_flt_reg (function)
macro_rules! Depcrate_isa_s390x_abiin_flt_reg {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"in_flt_reg"}
// Dependencies: {}
fn in_flt_reg (ty : Type) -> bool { match ty { types :: F32 | types :: F64 => true , _ => false , } }
};
}
