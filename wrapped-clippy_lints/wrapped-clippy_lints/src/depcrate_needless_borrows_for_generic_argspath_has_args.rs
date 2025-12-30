// Generated macro for path_has_args (function)
macro_rules! Depcrate_needless_borrows_for_generic_argspath_has_args {
() => {
// Module: crate::needless_borrows_for_generic_args
// Provides: {"path_has_args"}
// Dependencies: {}
fn path_has_args (p : & QPath < '_ >) -> bool { match * p { QPath :: Resolved (_ , Path { segments : [.. , s] , .. }) | QPath :: TypeRelative (_ , s) => s . args . is_some () , QPath :: Resolved (..) => false , } }
};
}
