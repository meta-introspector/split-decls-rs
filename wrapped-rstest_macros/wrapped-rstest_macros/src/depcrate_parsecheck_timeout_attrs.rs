// Generated macro for check_timeout_attrs (function)
macro_rules! Depcrate_parsecheck_timeout_attrs {
() => {
// Module: crate::parse
// Provides: {"check_timeout_attrs"}
// Dependencies: {}
pub (crate) fn check_timeout_attrs (item_fn : & mut ItemFn) -> Result < () , ErrorsVec > { let mut checker = CheckTimeoutAttributesFunction :: default () ; checker . visit_item_fn_mut (item_fn) ; checker . take () }
};
}
