// Generated macro for VectorN (type)
macro_rules! Depcrate_base_aliasVectorN {
() => {
// Module: crate::base::alias
// Provides: {"VectorN"}
// Dependencies: {}
# [doc = " An owned matrix column-major matrix with `R` rows and `C` columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated (note = "use SVector for a statically-sized matrix using integer dimensions, or OVector for an owned matrix using types as dimensions.")] pub type VectorN < T , D > = Matrix < T , D , U1 , Owned < T , D , U1 > > ;
};
}
