// Generated macro for check_path (function)
macro_rules! Depcrate_use_selfcheck_path {
() => {
// Module: crate::use_self
// Provides: {"check_path"}
// Dependencies: {}
fn check_path (cx : & LateContext < '_ > , path : & Path < '_ >) { match path . res { Res :: Def (DefKind :: Ctor (CtorOf :: Variant , _) | DefKind :: Variant , ..) => { lint_path_to_variant (cx , path) ; } , Res :: Def (DefKind :: Ctor (CtorOf :: Struct , _) | DefKind :: Struct , ..) => span_lint (cx , path . span) , _ => () , } }
};
}
