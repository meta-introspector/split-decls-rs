// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { crate :: { prelude :: * , PodG1 } , ark_bn254 :: g1 :: G1Affine , ark_ec :: AffineRepr , ark_serialize :: { CanonicalSerialize , Compress } , } ; # [test] fn zero_serialization_test () { let zero = G1Affine :: zero () ; let mut result_point_data = [0u8 ; 64] ; zero . x . serialize_with_mode (& mut result_point_data [.. 32] , Compress :: No) . map_err (| _ | AltBn128Error :: InvalidInputData) . unwrap () ; zero . y . serialize_with_mode (& mut result_point_data [32 ..] , Compress :: No) . map_err (| _ | AltBn128Error :: InvalidInputData) . unwrap () ; assert_eq ! (result_point_data , [0u8 ; 64]) ; let p : G1Affine = PodG1 (result_point_data [.. 64] . try_into () . unwrap ()) . try_into () . unwrap () ; assert_eq ! (p , zero) ; } }
};
}
