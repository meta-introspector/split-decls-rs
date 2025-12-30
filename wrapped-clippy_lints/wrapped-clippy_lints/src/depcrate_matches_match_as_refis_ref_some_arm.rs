// Generated macro for is_ref_some_arm (function)
macro_rules! Depcrate_matches_match_as_refis_ref_some_arm {
() => {
// Module: crate::matches::match_as_ref
// Provides: {"is_ref_some_arm"}
// Dependencies: {}
fn is_ref_some_arm (cx : & LateContext < '_ > , arm : & Arm < '_ >) -> Option < Mutability > { if let PatKind :: TupleStruct (ref qpath , [first_pat , ..] , _) = arm . pat . kind && is_res_lang_ctor (cx , cx . qpath_res (qpath , arm . pat . hir_id) , LangItem :: OptionSome) && let PatKind :: Binding (BindingMode (ByRef :: Yes (mutabl) , _) , .. , ident , _) = first_pat . kind && let ExprKind :: Call (e , [arg]) = peel_blocks (arm . body) . kind && is_res_lang_ctor (cx , path_res (cx , e) , LangItem :: OptionSome) && let ExprKind :: Path (QPath :: Resolved (_ , path2)) = arg . kind && path2 . segments . len () == 1 && ident . name == path2 . segments [0] . ident . name { return Some (mutabl) ; } None }
};
}
