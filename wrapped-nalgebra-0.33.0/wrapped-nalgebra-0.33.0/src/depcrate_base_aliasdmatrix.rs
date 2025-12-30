// Generated macro for DMatrix (type)
macro_rules! Depcrate_base_aliasDMatrix {
() => {
// Module: crate::base::alias
// Provides: {"DMatrix"}
// Dependencies: {}
# [doc = " A dynamically sized column-major matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type DMatrix < T > = Matrix < T , Dyn , Dyn , VecStorage < T , Dyn , Dyn > > ;
};
}
