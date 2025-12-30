// Generated macro for is_local_used_except (function)
macro_rules! Depcrate_shadowis_local_used_except {
() => {
// Module: crate::shadow
// Provides: {"is_local_used_except"}
// Dependencies: {}
# [doc = " Checks if the given local is used, except for in child expression of `except`."] # [doc = ""] # [doc = " This is a version of [`is_local_used`](clippy_utils::visitors::is_local_used), used to"] # [doc = " implement the fix for <https://github.com/rust-lang/rust-clippy/issues/10780>."] pub fn is_local_used_except < 'tcx > (cx : & LateContext < 'tcx > , visitable : impl Visitable < 'tcx > , id : HirId , except : Option < HirId > ,) -> bool { for_each_expr (cx , visitable , | e | { if except . is_some_and (| it | it == e . hir_id) { ControlFlow :: Continue (Descend :: No) } else if e . res_local_id () == Some (id) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (Descend :: Yes) } }) . is_some () }
};
}
