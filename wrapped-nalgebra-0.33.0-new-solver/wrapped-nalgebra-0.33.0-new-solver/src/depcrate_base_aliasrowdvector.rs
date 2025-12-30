// Generated macro for RowDVector (type)
macro_rules! Depcrate_base_aliasRowDVector {
() => {
// Module: crate::base::alias
// Provides: {"RowDVector"}
// Dependencies: {}
# [doc = " A dynamically sized row vector."] # [cfg (any (feature = "std" , feature = "alloc"))] pub type RowDVector < T > = Matrix < T , U1 , Dyn , VecStorage < T , U1 , Dyn > > ;
};
}
