// Generated macro for impl_209 (impl)
macro_rules! Depcrate_typesimpl_209 {
() => {
// Module: crate::types
// Provides: {"impl_209"}
// Dependencies: {}
impl Clone for FluentValue < '_ > { fn clone (& self) -> Self { match self { FluentValue :: String (s) => FluentValue :: String (s . clone ()) , FluentValue :: Number (s) => FluentValue :: Number (s . clone ()) , FluentValue :: Custom (s) => { let new_value : Box < dyn FluentType + Send > = s . duplicate () ; FluentValue :: Custom (new_value) } FluentValue :: Error => FluentValue :: Error , FluentValue :: None => FluentValue :: None , } } }
};
}
