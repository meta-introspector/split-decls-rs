// Generated macro for impl_601 (impl)
macro_rules! Depcrate_serimpl_601 {
() => {
// Module: crate::ser
// Provides: {"impl_601"}
// Dependencies: {}
impl < 'de , S , K , V > MapVisitor < 'de , S , K , V > where S : From < Vec < (K , V) > > , K : Deserialize < 'de > , V : Deserialize < 'de > , { pub (crate) fn new () -> MapVisitor < 'de , S , K , V > { MapVisitor { phantom_s : PhantomData , phantom_k : PhantomData , phantom_v : PhantomData , phantom_lifetime : PhantomData , } } }
};
}
