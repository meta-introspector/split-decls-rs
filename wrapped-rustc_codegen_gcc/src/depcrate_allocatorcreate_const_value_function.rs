// Generated macro for create_const_value_function (function)
macro_rules! Depcrate_allocatorcreate_const_value_function {
() => {
// Module: crate::allocator
// Provides: {"create_const_value_function"}
// Dependencies: {}
fn create_const_value_function (tcx : TyCtxt < '_ > , context : & Context < '_ > , name : & str , output : Type < '_ > , value : RValue < '_ > ,) { let func = context . new_function (None , FunctionType :: Exported , output , & [] , name , false) ; # [cfg (feature = "master")] { func . add_attribute (FnAttribute :: Visibility (symbol_visibility_to_gcc (tcx . sess . default_visibility () ,))) ; func . add_attribute (FnAttribute :: Inline) ; } if tcx . sess . must_emit_unwind_tables () { } let block = func . new_block ("entry") ; block . end_with_return (None , value) ; }
};
}
