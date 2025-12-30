// Generated macro for f16_builtin (function)
macro_rules! Depcrate_intrinsicf16_builtin {
() => {
// Module: crate::intrinsic
// Provides: {"f16_builtin"}
// Dependencies: {}
fn f16_builtin < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , name : Symbol , args : & [OperandRef < 'tcx , RValue < 'gcc > >] ,) -> RValue < 'gcc > { let f32_type = cx . type_f32 () ; let builtin_name = match name { sym :: ceilf16 => "__builtin_ceilf" , sym :: copysignf16 => "__builtin_copysignf" , sym :: floorf16 => "__builtin_floorf" , sym :: fmaf16 => "fmaf" , sym :: maxnumf16 => "__builtin_fmaxf" , sym :: minnumf16 => "__builtin_fminf" , sym :: powf16 => "__builtin_powf" , sym :: powif16 => { let func = cx . context . get_builtin_function ("__builtin_powif") ; let arg0 = cx . context . new_cast (None , args [0] . immediate () , f32_type) ; let args = [arg0 , args [1] . immediate ()] ; let result = cx . context . new_call (None , func , & args) ; return cx . context . new_cast (None , result , cx . type_f16 ()) ; } sym :: roundf16 => "__builtin_roundf" , sym :: round_ties_even_f16 => "__builtin_rintf" , sym :: sqrtf16 => "__builtin_sqrtf" , sym :: truncf16 => "__builtin_truncf" , _ => unreachable ! () , } ; let func = cx . context . get_builtin_function (builtin_name) ; let args : Vec < _ > = args . iter () . map (| arg | cx . context . new_cast (None , arg . immediate () , f32_type)) . collect () ; let result = cx . context . new_call (None , func , & args) ; cx . context . new_cast (None , result , cx . type_f16 ()) }
};
}
