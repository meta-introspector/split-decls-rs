// Generated macro for check (function)
macro_rules! Depcrate_casts_cast_enum_constructorcheck {
() => {
// Module: crate::casts::cast_enum_constructor
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , cast_expr : & Expr < '_ > , cast_from : Ty < '_ >) { if matches ! (cast_from . kind () , ty :: FnDef (..)) && let ExprKind :: Path (path) = & cast_expr . kind && let Res :: Def (DefKind :: Ctor (CtorOf :: Variant , CtorKind :: Fn) , _) = cx . qpath_res (path , cast_expr . hir_id) { span_lint (cx , CAST_ENUM_CONSTRUCTOR , expr . span , "cast of an enum tuple constructor to an integer" ,) ; } }
};
}
