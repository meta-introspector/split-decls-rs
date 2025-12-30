// Generated macro for ScalarBits (type)
macro_rules! Depcrate_scalarScalarBits {
() => {
// Module: crate::scalar
// Provides: {"ScalarBits"}
// Dependencies: {}
# [doc = " Bit representation of a scalar field element of a given curve."] # [cfg (feature = "bits")] pub type ScalarBits < C > = ff :: FieldBits < < Scalar < C > as ff :: PrimeFieldBits > :: ReprBits > ;
};
}
