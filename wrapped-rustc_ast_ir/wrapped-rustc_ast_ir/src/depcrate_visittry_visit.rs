// Generated macro for try_visit (macro)
macro_rules! Depcrate_visittry_visit {
() => {
// Module: crate::visit
// Provides: {"try_visit"}
// Dependencies: {}
# [macro_export] macro_rules ! try_visit { ($ e : expr) => { match $ crate :: visit :: VisitorResult :: branch ($ e) { core :: ops :: ControlFlow :: Continue (()) => () , # [allow (unreachable_code)] core :: ops :: ControlFlow :: Break (r) => { return $ crate :: visit :: VisitorResult :: from_residual (r) ; } } } ; }
};
}
