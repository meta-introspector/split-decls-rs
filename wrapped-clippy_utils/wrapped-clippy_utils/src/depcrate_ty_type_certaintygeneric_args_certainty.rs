// Generated macro for generic_args_certainty (function)
macro_rules! Depcrate_ty_type_certaintygeneric_args_certainty {
() => {
// Module: crate::ty::type_certainty
// Provides: {"generic_args_certainty"}
// Dependencies: {}
fn generic_args_certainty (cx : & LateContext < '_ > , args : & GenericArgs < '_ >) -> Certainty { let mut visitor = CertaintyVisitor :: new (cx) ; visitor . visit_generic_args (args) ; visitor . certainty }
};
}
