// Generated macro for type_zero_value (function)
macro_rules! Depcrate_commontype_zero_value {
() => {
// Module: crate::common
// Provides: {"type_zero_value"}
// Dependencies: {}
pub (crate) fn type_zero_value (bcx : & mut FunctionBuilder < '_ > , ty : Type) -> Value { if ty == types :: I128 { let zero = bcx . ins () . iconst (types :: I64 , 0) ; bcx . ins () . iconcat (zero , zero) } else { bcx . ins () . iconst (ty , 0) } }
};
}
