// Generated macro for add_top_filter (function)
macro_rules! Depcrate_deriveadd_top_filter {
() => {
// Module: crate::derive
// Provides: {"add_top_filter"}
// Dependencies: {}
# [doc = " Apply the filter at the top level if provided."] fn add_top_filter (filter : Vec < Expr > , parts : ImplParts) -> ImplParts { let (params , strat , ctor) = parts ; let (strat , ctor) = add_filter_self (filter , (strat , ctor)) ; (params , strat , ctor) }
};
}
