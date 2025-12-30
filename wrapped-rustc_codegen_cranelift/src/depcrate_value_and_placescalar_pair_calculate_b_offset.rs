// Generated macro for scalar_pair_calculate_b_offset (function)
macro_rules! Depcrate_value_and_placescalar_pair_calculate_b_offset {
() => {
// Module: crate::value_and_place
// Provides: {"scalar_pair_calculate_b_offset"}
// Dependencies: {}
fn scalar_pair_calculate_b_offset (tcx : TyCtxt < '_ > , a_scalar : Scalar , b_scalar : Scalar) -> Offset32 { let b_offset = a_scalar . size (& tcx) . align_to (b_scalar . align (& tcx) . abi) ; Offset32 :: new (b_offset . bytes () . try_into () . unwrap ()) }
};
}
