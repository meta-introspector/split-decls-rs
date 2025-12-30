// Generated macro for CsMatrix (struct)
macro_rules! Depcrate_sparse_cs_matrixCsMatrix {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"CsMatrix"}
// Dependencies: {}
# [doc = " A compressed sparse column matrix."] # [derive (Clone , Debug , PartialEq)] pub struct CsMatrix < T : Scalar , R : Dim = Dyn , C : Dim = Dyn , S : CsStorage < T , R , C > = CsVecStorage < T , R , C > , > { pub (crate) data : S , _phantoms : PhantomData < (T , R , C) > , }
};
}
