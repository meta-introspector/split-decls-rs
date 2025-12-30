// Generated macro for in_vec_reg (function)
macro_rules! Depcrate_isa_s390x_abiin_vec_reg {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"in_vec_reg"}
// Dependencies: {}
fn in_vec_reg (ty : Type) -> bool { ty . is_vector () && ty . bits () == 128 }
};
}
