// Generated macro for as_ref_some_arm (function)
macro_rules! Depcrate_matches_match_as_refas_ref_some_arm {
() => {
// Module: crate::matches::match_as_ref
// Provides: {"as_ref_some_arm"}
// Dependencies: {}
fn as_ref_some_arm (cx : & LateContext < '_ > , arm : & Arm < '_ >) -> Option < Mutability > { if let Some ([first_pat , ..]) = as_some_pattern (cx , arm . pat) && let PatKind :: Binding (BindingMode (ByRef :: Yes (_ , mutabl) , _) , .. , ident , _) = first_pat . kind && let Some (arg) = as_some_expr (cx , peel_blocks (arm . body)) && let ExprKind :: Path (QPath :: Resolved (_ , path2)) = arg . kind && path2 . segments . len () == 1 && ident . name == path2 . segments [0] . ident . name { return Some (mutabl) ; } None }
};
}
