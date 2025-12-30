// Generated macro for impl_208 (impl)
macro_rules! Depcrate_typesimpl_208 {
() => {
// Module: crate::types
// Provides: {"impl_208"}
// Dependencies: {}
impl PartialEq for FluentValue < '_ > { fn eq (& self , other : & Self) -> bool { match (self , other) { (FluentValue :: String (s) , FluentValue :: String (s2)) => s == s2 , (FluentValue :: Number (s) , FluentValue :: Number (s2)) => s == s2 , (FluentValue :: Custom (s) , FluentValue :: Custom (s2)) => s == s2 , _ => false , } } }
};
}
