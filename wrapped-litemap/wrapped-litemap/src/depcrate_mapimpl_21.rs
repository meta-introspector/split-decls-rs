// Generated macro for impl_21 (impl)
macro_rules! Depcrate_mapimpl_21 {
() => {
// Module: crate::map
// Provides: {"impl_21"}
// Dependencies: {}
impl < K : ? Sized , V : ? Sized , S > LiteMap < K , V , S > where S : StoreConstEmpty < K , V > , { # [doc = " Create a new empty [`LiteMap`]"] pub const fn new () -> Self { Self { values : S :: EMPTY , _key_type : PhantomData , _value_type : PhantomData , } } }
};
}
