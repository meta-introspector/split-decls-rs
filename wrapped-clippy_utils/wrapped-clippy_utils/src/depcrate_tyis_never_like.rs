// Generated macro for is_never_like (function)
macro_rules! Depcrate_tyis_never_like {
() => {
// Module: crate::ty
// Provides: {"is_never_like"}
// Dependencies: {}
# [doc = " Returns whether `ty` is never-like; i.e., `!` (never) or an enum with zero variants."] pub fn is_never_like (ty : Ty < '_ >) -> bool { ty . is_never () || (ty . is_enum () && ty . ty_adt_def () . is_some_and (| def | def . variants () . is_empty ())) }
};
}
