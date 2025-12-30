// Generated macro for simd_pair_for_each_lane_typed (function)
macro_rules! Depcrate_intrinsicssimd_pair_for_each_lane_typed {
() => {
// Module: crate::intrinsics
// Provides: {"simd_pair_for_each_lane_typed"}
// Dependencies: {}
fn simd_pair_for_each_lane_typed < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , x : CValue < 'tcx > , y : CValue < 'tcx > , ret : CPlace < 'tcx > , f : & dyn Fn (& mut FunctionCx < '_ , '_ , 'tcx > , CValue < 'tcx > , CValue < 'tcx >) -> CValue < 'tcx > ,) { assert_eq ! (x . layout () , y . layout ()) ; let layout = x . layout () ; let (lane_count , _lane_ty) = layout . ty . simd_size_and_type (fx . tcx) ; let (ret_lane_count , _ret_lane_ty) = ret . layout () . ty . simd_size_and_type (fx . tcx) ; assert_eq ! (lane_count , ret_lane_count) ; for lane_idx in 0 .. lane_count { let x_lane = x . value_lane (fx , lane_idx) ; let y_lane = y . value_lane (fx , lane_idx) ; let res_lane = f (fx , x_lane , y_lane) ; ret . place_lane (fx , lane_idx) . write_cvalue (fx , res_lane) ; } }
};
}
