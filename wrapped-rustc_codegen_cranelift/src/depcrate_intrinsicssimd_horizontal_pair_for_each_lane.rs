// Generated macro for simd_horizontal_pair_for_each_lane (function)
macro_rules! Depcrate_intrinsicssimd_horizontal_pair_for_each_lane {
() => {
// Module: crate::intrinsics
// Provides: {"simd_horizontal_pair_for_each_lane"}
// Dependencies: {}
fn simd_horizontal_pair_for_each_lane < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , x : CValue < 'tcx > , y : CValue < 'tcx > , ret : CPlace < 'tcx > , f : & dyn Fn (& mut FunctionCx < '_ , '_ , 'tcx > , Ty < 'tcx > , Ty < 'tcx > , Value , Value) -> Value ,) { assert_eq ! (x . layout () , y . layout ()) ; let layout = x . layout () ; let (lane_count , lane_ty) = layout . ty . simd_size_and_type (fx . tcx) ; let lane_layout = fx . layout_of (lane_ty) ; let (ret_lane_count , ret_lane_ty) = ret . layout () . ty . simd_size_and_type (fx . tcx) ; let ret_lane_layout = fx . layout_of (ret_lane_ty) ; assert_eq ! (lane_count , ret_lane_count) ; for lane_idx in 0 .. lane_count { let src = if lane_idx < (lane_count / 2) { x } else { y } ; let src_idx = lane_idx % (lane_count / 2) ; let lhs_lane = src . value_lane (fx , src_idx * 2) . load_scalar (fx) ; let rhs_lane = src . value_lane (fx , src_idx * 2 + 1) . load_scalar (fx) ; let res_lane = f (fx , lane_layout . ty , ret_lane_layout . ty , lhs_lane , rhs_lane) ; let res_lane = CValue :: by_val (res_lane , ret_lane_layout) ; ret . place_lane (fx , lane_idx) . write_cvalue (fx , res_lane) ; } }
};
}
