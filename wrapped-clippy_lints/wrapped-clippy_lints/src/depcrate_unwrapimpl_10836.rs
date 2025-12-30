// Generated macro for impl_10836 (impl)
macro_rules! Depcrate_unwrapimpl_10836 {
() => {
// Module: crate::unwrap
// Provides: {"impl_10836"}
// Dependencies: {}
impl UnwrappableKind { fn success_variant_pattern (self) -> & 'static str { match self { UnwrappableKind :: Option => "Some(<item>)" , UnwrappableKind :: Result => "Ok(<item>)" , } } fn error_variant_pattern (self) -> & 'static str { match self { UnwrappableKind :: Option => "None" , UnwrappableKind :: Result => "Err(<item>)" , } } }
};
}
