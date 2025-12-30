// Generated macro for impl_145 (impl)
macro_rules! Depcrate_data_providerimpl_145 {
() => {
// Module: crate::data_provider
// Provides: {"impl_145"}
// Dependencies: {}
impl < M , P > DataProviderWithMarker < M , P > where M : DataMarker , P : DataProvider < M > , { # [doc = " Creates a [`DataProviderWithMarker`] from a [`DataProvider`] with a [`DataMarker`]."] pub const fn new (inner : P) -> Self { Self { inner , _marker : PhantomData , } } }
};
}
