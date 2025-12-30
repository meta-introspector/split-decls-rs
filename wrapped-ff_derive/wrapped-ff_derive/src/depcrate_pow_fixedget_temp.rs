// Generated macro for get_temp (function)
macro_rules! Depcrate_pow_fixedget_temp {
() => {
// Module: crate::pow_fixed
// Provides: {"get_temp"}
// Dependencies: {}
# [doc = " Returns t{n} as an ident."] fn get_temp (n : usize) -> Ident { Ident :: new (& format ! ("t{}" , n) , proc_macro2 :: Span :: call_site ()) }
};
}
