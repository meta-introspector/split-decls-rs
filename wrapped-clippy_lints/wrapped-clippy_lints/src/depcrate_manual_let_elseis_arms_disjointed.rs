// Generated macro for is_arms_disjointed (function)
macro_rules! Depcrate_manual_let_elseis_arms_disjointed {
() => {
// Module: crate::manual_let_else
// Provides: {"is_arms_disjointed"}
// Dependencies: {}
# [doc = " Checks if the patterns of the arms are disjointed. Currently, we only support patterns of simple"] # [doc = " enum variants without nested patterns or bindings."] # [doc = ""] # [doc = " TODO: Support more complex patterns."] fn is_arms_disjointed (cx : & LateContext < '_ > , arm1 : & Arm < '_ > , arm2 : & Arm < '_ >) -> bool { if arm1 . guard . is_some () || arm2 . guard . is_some () { return false ; } if ! is_enum_variant (cx , arm1 . pat) || ! is_enum_variant (cx , arm2 . pat) { return false ; } true }
};
}
