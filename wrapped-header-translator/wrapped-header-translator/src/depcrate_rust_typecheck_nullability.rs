// Generated macro for check_nullability (function)
macro_rules! Depcrate_rust_typecheck_nullability {
() => {
// Module: crate::rust_type
// Provides: {"check_nullability"}
// Dependencies: {}
fn check_nullability (ty : & Type < '_ > , new : Option < Nullability >) -> Nullability { let on_ty = ty . get_nullability () ; if new != on_ty { error ! (? ty , ? on_ty , ? new , "failed parsing nullability") ; } new . unwrap_or (Nullability :: Unspecified) }
};
}
