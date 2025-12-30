// Generated macro for simd_reduce (function)
macro_rules! Depcrate_intrinsicssimd_reduce {
() => {
// Module: crate::intrinsics
// Provides: {"simd_reduce"}
// Dependencies: {}
fn simd_reduce < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , val : CValue < 'tcx > , acc : Option < Value > , ret : CPlace < 'tcx > , f : & dyn Fn (& mut FunctionCx < '_ , '_ , 'tcx > , Ty < 'tcx > , Value , Value) -> Value ,) { let (lane_count , lane_ty) = val . layout () . ty . simd_size_and_type (fx . tcx) ; let lane_layout = fx . layout_of (lane_ty) ; assert_eq ! (lane_layout , ret . layout ()) ; let (mut res_val , start_lane) = if let Some (acc) = acc { (acc , 0) } else { (val . value_lane (fx , 0) . load_scalar (fx) , 1) } ; for lane_idx in start_lane .. lane_count { let lane = val . value_lane (fx , lane_idx) . load_scalar (fx) ; res_val = f (fx , lane_layout . ty , res_val , lane) ; } let res = CValue :: by_val (res_val , lane_layout) ; ret . write_cvalue (fx , res) ; }
};
}
