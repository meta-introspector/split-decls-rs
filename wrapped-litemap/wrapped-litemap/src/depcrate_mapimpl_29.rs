// Generated macro for impl_29 (impl)
macro_rules! Depcrate_mapimpl_29 {
() => {
// Module: crate::map
// Provides: {"impl_29"}
// Dependencies: {}
impl < K , V , S > Default for LiteMap < K , V , S > where S : Store < K , V > + Default , { fn default () -> Self { Self { values : S :: default () , _key_type : PhantomData , _value_type : PhantomData , } } }
};
}
