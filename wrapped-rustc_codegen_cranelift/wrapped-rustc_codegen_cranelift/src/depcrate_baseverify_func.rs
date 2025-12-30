// Generated macro for verify_func (function)
macro_rules! Depcrate_baseverify_func {
() => {
// Module: crate::base
// Provides: {"verify_func"}
// Dependencies: {}
fn verify_func (tcx : TyCtxt < '_ > , writer : & crate :: pretty_clif :: CommentWriter , func : & Function) { if ! enable_verifier (tcx . sess) { return ; } tcx . prof . generic_activity ("verify clif ir") . run (| | { let flags = cranelift_codegen :: settings :: Flags :: new (cranelift_codegen :: settings :: builder ()) ; match cranelift_codegen :: verify_function (& func , & flags) { Ok (_) => { } Err (err) => { tcx . dcx () . err (format ! ("{:?}" , err)) ; let pretty_error = cranelift_codegen :: print_errors :: pretty_verifier_error (& func , Some (Box :: new (writer)) , err ,) ; tcx . dcx () . fatal (format ! ("cranelift verify error:\n{}" , pretty_error)) ; } } }) ; }
};
}
