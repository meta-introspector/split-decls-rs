// Generated macro for impl_947 (impl)
macro_rules! Depcrate_util_spanned_valueimpl_947 {
() => {
// Module: crate::util::spanned_value
// Provides: {"impl_947"}
// Dependencies: {}
impl < T > SpannedValue < T > { pub fn new (value : T , span : Span) -> Self { SpannedValue { value , span } } # [doc = " Get the source code location referenced by this struct."] pub fn span (& self) -> Span { self . span } # [doc = " Apply a mapping function to a reference to the spanned value."] pub fn map_ref < U > (& self , map_fn : impl FnOnce (& T) -> U) -> SpannedValue < U > { SpannedValue :: new (map_fn (& self . value) , self . span) } # [doc = " Gets the inner value, consuming `self` in the process."] pub fn into_inner (self) -> T { self . value } }
};
}
