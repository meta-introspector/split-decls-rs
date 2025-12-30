// Generated macro for codegen_intrinsic_call (function)
macro_rules! Depcrate_intrinsicscodegen_intrinsic_call {
() => {
// Module: crate::intrinsics
// Provides: {"codegen_intrinsic_call"}
// Dependencies: {}
pub (crate) fn codegen_intrinsic_call < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , instance : Instance < 'tcx > , args : & [Spanned < mir :: Operand < 'tcx > >] , destination : CPlace < 'tcx > , target : Option < BasicBlock > , source_info : mir :: SourceInfo ,) -> Result < () , Instance < 'tcx > > { let intrinsic = fx . tcx . item_name (instance . def_id ()) ; let instance_args = instance . args ; if intrinsic . as_str () . starts_with ("simd_") { self :: simd :: codegen_simd_intrinsic_call (fx , intrinsic , instance_args , args , destination , target . expect ("target for simd intrinsic") , source_info . span ,) ; } else if codegen_float_intrinsic_call (fx , intrinsic , args , destination) { let ret_block = fx . get_block (target . expect ("target for float intrinsic")) ; fx . bcx . ins () . jump (ret_block , & []) ; } else { codegen_regular_intrinsic_call (fx , instance , intrinsic , instance_args , args , destination , target , source_info ,) ? ; } Ok (()) }
};
}
