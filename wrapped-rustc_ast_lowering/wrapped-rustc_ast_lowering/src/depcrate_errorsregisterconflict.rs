// Generated macro for RegisterConflict (struct)
macro_rules! Depcrate_errorsRegisterConflict {
() => {
// Module: crate::errors
// Provides: {"RegisterConflict"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_register_conflict)] pub (crate) struct RegisterConflict < 'a > { # [primary_span] # [label (ast_lowering_register1)] pub op_span1 : Span , # [label (ast_lowering_register2)] pub op_span2 : Span , pub reg1_name : & 'a str , pub reg2_name : & 'a str , # [help] pub in_out : Option < Span > , }
};
}
