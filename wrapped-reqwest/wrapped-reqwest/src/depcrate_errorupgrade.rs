// Generated macro for upgrade (function)
macro_rules! Depcrate_errorupgrade {
() => {
// Module: crate::error
// Provides: {"upgrade"}
// Dependencies: {}
pub (crate) fn upgrade < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Upgrade , Some (e)) }
};
}
