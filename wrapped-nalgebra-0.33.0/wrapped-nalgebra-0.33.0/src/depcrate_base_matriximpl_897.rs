// Generated macro for impl_897 (impl)
macro_rules! Depcrate_base_matriximpl_897 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_897"}
// Dependencies: {}
impl < T , R , C , S > Default for Matrix < T , R , C , S > where T : Scalar , R : Dim , C : Dim , S : Default , { fn default () -> Self { Matrix { data : Default :: default () , _phantoms : PhantomData , } } }
};
}
