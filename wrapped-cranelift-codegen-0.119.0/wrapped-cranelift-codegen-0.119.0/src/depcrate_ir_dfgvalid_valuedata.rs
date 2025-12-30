// Generated macro for valid_valuedata (function)
macro_rules! Depcrate_ir_dfgvalid_valuedata {
() => {
// Module: crate::ir::dfg
// Provides: {"valid_valuedata"}
// Dependencies: {}
# [doc = " Check for non-values."] fn valid_valuedata (data : ValueDataPacked) -> bool { let data = ValueData :: from (data) ; if let ValueData :: Alias { ty : types :: INVALID , original , } = ValueData :: from (data) { if original == Value :: reserved_value () { return false ; } } true }
};
}
