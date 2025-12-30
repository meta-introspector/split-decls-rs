// Generated macro for is_non_trait_box (function)
macro_rules! Depcrate_escapeis_non_trait_box {
() => {
// Module: crate::escape
// Provides: {"is_non_trait_box"}
// Dependencies: {}
fn is_non_trait_box (ty : Ty < '_ >) -> bool { ty . boxed_ty () . is_some_and (| boxed | ! boxed . is_trait ()) }
};
}
