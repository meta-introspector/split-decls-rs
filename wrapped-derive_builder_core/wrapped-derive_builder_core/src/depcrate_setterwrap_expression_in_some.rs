// Generated macro for wrap_expression_in_some (function)
macro_rules! Depcrate_setterwrap_expression_in_some {
() => {
// Module: crate::setter
// Provides: {"wrap_expression_in_some"}
// Dependencies: {}
# [doc = " Returns expression wrapping `bare_value` in `Some`"] fn wrap_expression_in_some (crate_root : & syn :: Path , bare_value : impl ToTokens) -> TokenStream { quote ! (# crate_root :: export :: core :: option :: Option :: Some (# bare_value)) }
};
}
