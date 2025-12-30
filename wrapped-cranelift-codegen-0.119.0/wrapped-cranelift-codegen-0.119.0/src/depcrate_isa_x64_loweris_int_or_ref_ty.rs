// Generated macro for is_int_or_ref_ty (function)
macro_rules! Depcrate_isa_x64_loweris_int_or_ref_ty {
() => {
// Module: crate::isa::x64::lower
// Provides: {"is_int_or_ref_ty"}
// Dependencies: {}
fn is_int_or_ref_ty (ty : Type) -> bool { match ty { types :: I8 | types :: I16 | types :: I32 | types :: I64 => true , _ => false , } }
};
}
