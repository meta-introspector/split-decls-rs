// Generated macro for is_ty_exported (function)
macro_rules! Depcrate_iter_without_into_iteris_ty_exported {
() => {
// Module: crate::iter_without_into_iter
// Provides: {"is_ty_exported"}
// Dependencies: {}
fn is_ty_exported (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { ty . ty_adt_def () . and_then (| adt | adt . did () . as_local ()) . is_some_and (| did | cx . effective_visibilities . is_exported (did)) }
};
}
