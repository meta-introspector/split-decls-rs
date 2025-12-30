// Generated macro for DVector (type)
macro_rules! Depcrate_base_aliasDVector {
() => {
// Module: crate::base::alias
// Provides: {"DVector"}
// Dependencies: {}
# [doc = " A dynamically sized column vector."] # [cfg (any (feature = "std" , feature = "alloc"))] pub type DVector < T > = Matrix < T , Dyn , U1 , VecStorage < T , Dyn , U1 > > ;
};
}
