// Generated macro for impl_1182 (impl)
macro_rules! Depcrate_graphmapimpl_1182 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1182"}
// Dependencies: {}
impl < N : Eq + Hash + fmt :: Debug , E : fmt :: Debug , Ty : EdgeType , S : BuildHasher > fmt :: Debug for GraphMap < N , E , Ty , S > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . nodes . fmt (f) } }
};
}
