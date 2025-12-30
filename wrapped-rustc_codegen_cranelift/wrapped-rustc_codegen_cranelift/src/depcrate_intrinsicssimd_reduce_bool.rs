// Generated macro for simd_reduce_bool (function)
macro_rules! Depcrate_intrinsicssimd_reduce_bool {
() => {
// Module: crate::intrinsics
// Provides: {"simd_reduce_bool"}
// Dependencies: {}
fn simd_reduce_bool < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , val : CValue < 'tcx > , ret : CPlace < 'tcx > , f : & dyn Fn (& mut FunctionCx < '_ , '_ , 'tcx > , Value , Value) -> Value ,) { let (lane_count , _lane_ty) = val . layout () . ty . simd_size_and_type (fx . tcx) ; assert ! (ret . layout () . ty . is_bool ()) ; let res_val = val . value_lane (fx , 0) . load_scalar (fx) ; let mut res_val = fx . bcx . ins () . band_imm (res_val , 1) ; for lane_idx in 1 .. lane_count { let lane = val . value_lane (fx , lane_idx) . load_scalar (fx) ; let lane = fx . bcx . ins () . band_imm (lane , 1) ; res_val = f (fx , res_val , lane) ; } let res_val = if fx . bcx . func . dfg . value_type (res_val) != types :: I8 { fx . bcx . ins () . ireduce (types :: I8 , res_val) } else { res_val } ; let res = CValue :: by_val (res_val , ret . layout ()) ; ret . write_cvalue (fx , res) ; }
};
}
