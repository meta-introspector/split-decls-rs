// Generated macro for get_simple_function_f128 (function)
macro_rules! Depcrate_intrinsicget_simple_function_f128 {
() => {
// Module: crate::intrinsic
// Provides: {"get_simple_function_f128"}
// Dependencies: {}
fn get_simple_function_f128 < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , name : Symbol ,) -> Option < Function < 'gcc > > { if ! cx . supports_f128_type { return None ; } let f128_type = cx . type_f128 () ; let func_name = match name { sym :: ceilf128 => "ceilf128" , sym :: floorf128 => "floorf128" , sym :: truncf128 => "truncf128" , sym :: roundf128 => "roundf128" , sym :: round_ties_even_f128 => "roundevenf128" , sym :: sqrtf128 => "sqrtf128" , _ => return None , } ; Some (cx . context . new_function (None , FunctionType :: Extern , f128_type , & [cx . context . new_parameter (None , f128_type , "a")] , func_name , false ,)) }
};
}
