// Generated macro for impl_96 (impl)
macro_rules! Depcrate_frontendimpl_96 {
() => {
// Module: crate::frontend
// Provides: {"impl_96"}
// Dependencies: {}
impl < B > Pattern < B > where for < 'b > B : PatternBackend < Error < 'b > = Infallible > , { # [doc = " Returns a [`Writeable`] that interpolates items from the given replacement provider"] # [doc = " into this pattern string."] pub fn interpolate < 'a , P > (& 'a self , value_provider : P) -> impl Writeable + fmt :: Display + 'a where P : PlaceholderValueProvider < B :: PlaceholderKey < 'a > , Error = B :: Error < 'a > > + 'a , { TryWriteableInfallibleAsWriteable (WriteablePattern :: < B , P > { store : & self . store , value_provider , }) } # [cfg (feature = "alloc")] # [doc = " Interpolates the pattern directly to a string."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] pub fn interpolate_to_string < 'a , P > (& 'a self , value_provider : P) -> String where P : PlaceholderValueProvider < B :: PlaceholderKey < 'a > , Error = B :: Error < 'a > > + 'a , { self . interpolate (value_provider) . write_to_string () . into_owned () } }
};
}
