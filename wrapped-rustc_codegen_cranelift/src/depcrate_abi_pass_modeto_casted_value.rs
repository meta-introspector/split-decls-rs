// Generated macro for to_casted_value (function)
macro_rules! Depcrate_abi_pass_modeto_casted_value {
() => {
// Module: crate::abi::pass_mode
// Provides: {"to_casted_value"}
// Dependencies: {}
pub (super) fn to_casted_value < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , arg : CValue < 'tcx > , cast : & CastTarget ,) -> SmallVec < [Value ; 2] > { let (ptr , meta) = arg . force_stack (fx) ; assert ! (meta . is_none ()) ; cast_target_to_abi_params (cast) . into_iter () . map (| (offset , param) | { let val = ptr . offset_i64 (fx , offset . bytes () as i64) . load (fx , param . value_type , MemFlags :: new () ,) ; val }) . collect () }
};
}
