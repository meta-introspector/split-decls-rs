// Generated macro for impl_360 (impl)
macro_rules! Depcrate_mapimpl_360 {
() => {
// Module: crate::map
// Provides: {"impl_360"}
// Dependencies: {}
impl < K : Debug , V : Debug , S , A : Allocator > fmt :: Display for OccupiedError < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to insert {:?}, key {:?} already exists with value {:?}" , self . value , self . entry . key () , self . entry . get () ,) } }
};
}
