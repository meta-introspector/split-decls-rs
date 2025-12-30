// Generated macro for impl_830 (impl)
macro_rules! Depcrate_base_editionimpl_830 {
() => {
// Module: crate::base::edition
// Provides: {"impl_830"}
// Dependencies: {}
# [doc = " Extend the number of rows of the `Vector` with elements from"] # [doc = " a given iterator."] # [cfg (any (feature = "std" , feature = "alloc"))] impl < T , S > Extend < T > for Matrix < T , Dyn , U1 , S > where T : Scalar , S : Extend < T > , { # [doc = " Extend the number of rows of a `Vector` with elements"] # [doc = " from the given iterator."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::DVector;"] # [doc = " let mut vector = DVector::from_vec(vec![0, 1, 2]);"] # [doc = " vector.extend(vec![3, 4, 5]);"] # [doc = " assert!(vector.eq(&DVector::from_vec(vec![0, 1, 2, 3, 4, 5])));"] # [doc = " ```"] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . data . extend (iter) ; } }
};
}
