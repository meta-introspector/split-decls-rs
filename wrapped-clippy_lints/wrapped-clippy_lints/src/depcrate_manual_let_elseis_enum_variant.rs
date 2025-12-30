// Generated macro for is_enum_variant (function)
macro_rules! Depcrate_manual_let_elseis_enum_variant {
() => {
// Module: crate::manual_let_else
// Provides: {"is_enum_variant"}
// Dependencies: {}
# [doc = " Returns `true` if the given pattern is a variant of an enum."] pub fn is_enum_variant (cx : & LateContext < '_ > , pat : & Pat < '_ >) -> bool { let path = match pat . kind { PatKind :: Struct (ref qpath , fields , _) if fields . iter () . all (| field | is_wild (field . pat) || matches ! (field . pat . kind , PatKind :: Binding (..))) => { (qpath , pat . hir_id) } , PatKind :: TupleStruct (ref qpath , pats , _) if pats . iter () . all (| pat | is_wild (pat) || matches ! (pat . kind , PatKind :: Binding (..))) => { (qpath , pat . hir_id) } , PatKind :: Expr (e) if let Some ((qpath , id)) = e . opt_qpath () => (qpath , id) , _ => return false , } ; let res = path . res (cx) ; matches ! (res , Res :: Def (DefKind :: Variant , ..) | Res :: Def (DefKind :: Ctor (CtorOf :: Variant , _) , _)) }
};
}
