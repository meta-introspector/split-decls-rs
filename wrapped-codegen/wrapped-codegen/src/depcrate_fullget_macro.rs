// Generated macro for get_macro (function)
macro_rules! Depcrate_fullget_macro {
() => {
// Module: crate::full
// Provides: {"get_macro"}
// Dependencies: {}
pub fn get_macro () -> TokenStream { quote ! { # [cfg (feature = "full")] macro_rules ! full { ($ e : expr) => { $ e } ; } # [cfg (all (feature = "derive" , not (feature = "full")))] macro_rules ! full { ($ e : expr) => { unreachable ! () } ; } } }
};
}
