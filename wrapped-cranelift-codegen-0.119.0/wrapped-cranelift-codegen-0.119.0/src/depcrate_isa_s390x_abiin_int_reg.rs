// Generated macro for in_int_reg (function)
macro_rules! Depcrate_isa_s390x_abiin_int_reg {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"in_int_reg"}
// Dependencies: {}
# [doc = " ABI Register usage"] fn in_int_reg (ty : Type) -> bool { match ty { types :: I8 | types :: I16 | types :: I32 | types :: I64 => true , _ => false , } }
};
}
