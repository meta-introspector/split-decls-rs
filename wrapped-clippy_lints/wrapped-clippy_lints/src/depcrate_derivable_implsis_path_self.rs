// Generated macro for is_path_self (function)
macro_rules! Depcrate_derivable_implsis_path_self {
() => {
// Module: crate::derivable_impls
// Provides: {"is_path_self"}
// Dependencies: {}
fn is_path_self (e : & Expr < '_ >) -> bool { if let ExprKind :: Path (QPath :: Resolved (_ , p)) = e . kind { matches ! (p . res , Res :: SelfCtor (..) | Res :: Def (DefKind :: Ctor (..) , _)) } else { false } }
};
}
