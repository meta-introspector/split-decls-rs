// Generated macro for MapVisitor (struct)
macro_rules! Depcrate_serMapVisitor {
() => {
// Module: crate::ser
// Provides: {"MapVisitor"}
// Dependencies: {}
struct MapVisitor < 'de , S , K , V > where S : From < Vec < (K , V) > > , K : Deserialize < 'de > , V : Deserialize < 'de > , { phantom_s : PhantomData < S > , phantom_k : PhantomData < K > , phantom_v : PhantomData < V > , phantom_lifetime : PhantomData < & 'de () > , }
};
}
