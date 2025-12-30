// Generated macro for impl_39 (impl)
macro_rules! Depcrate_exemplar_charsimpl_39 {
() => {
// Module: crate::exemplar_chars
// Provides: {"impl_39"}
// Dependencies: {}
impl ExemplarCharactersBorrowed < 'static > { # [doc = " Cheaply converts a [`ExemplarCharactersBorrowed<'static>`] into a [`ExemplarCharacters`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`ExemplarCharacters`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`ExemplarCharactersBorrowed`]."] pub const fn static_to_owned (self) -> ExemplarCharacters { ExemplarCharacters { data : DataPayload :: from_static_ref (self . data) , } } }
};
}
