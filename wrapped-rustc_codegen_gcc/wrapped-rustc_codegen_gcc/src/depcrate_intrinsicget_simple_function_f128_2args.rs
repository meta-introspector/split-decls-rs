// Generated macro for get_simple_function_f128_2args (function)
macro_rules! Depcrate_intrinsicget_simple_function_f128_2args {
() => {
// Module: crate::intrinsic
// Provides: {"get_simple_function_f128_2args"}
// Dependencies: {}
fn get_simple_function_f128_2args < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , name : Symbol ,) -> Option < Function < 'gcc > > { if ! cx . supports_f128_type { return None ; } let f128_type = cx . type_f128 () ; let func_name = match name { sym :: maxnumf128 => "fmaxf128" , sym :: minnumf128 => "fminf128" , sym :: copysignf128 => "copysignf128" , _ => return None , } ; Some (cx . context . new_function (None , FunctionType :: Extern , f128_type , & [cx . context . new_parameter (None , f128_type , "a") , cx . context . new_parameter (None , f128_type , "b") ,] , func_name , false ,)) }
};
}
