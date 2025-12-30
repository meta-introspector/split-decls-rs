// Generated macro for InteriorMutableBorrowEscaping (struct)
macro_rules! Depcrate_errorsInteriorMutableBorrowEscaping {
() => {
// Module: crate::errors
// Provides: {"InteriorMutableBorrowEscaping"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (const_eval_interior_mutable_borrow_escaping , code = E0492)] # [note] # [note (const_eval_note2)] # [help] pub (crate) struct InteriorMutableBorrowEscaping { # [primary_span] # [label] pub span : Span , pub kind : ConstContext , }
};
}
