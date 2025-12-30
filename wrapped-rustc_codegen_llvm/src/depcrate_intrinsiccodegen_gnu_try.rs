// Generated macro for codegen_gnu_try (function)
macro_rules! Depcrate_intrinsiccodegen_gnu_try {
() => {
// Module: crate::intrinsic
// Provides: {"codegen_gnu_try"}
// Dependencies: {}
fn codegen_gnu_try < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , try_func : & 'll Value , data : & 'll Value , catch_func : & 'll Value , dest : PlaceRef < 'tcx , & 'll Value > ,) { let (llty , llfn) = get_rust_try_fn (bx , & mut | mut bx | { let then = bx . append_sibling_block ("then") ; let catch = bx . append_sibling_block ("catch") ; let try_func = llvm :: get_param (bx . llfn () , 0) ; let data = llvm :: get_param (bx . llfn () , 1) ; let catch_func = llvm :: get_param (bx . llfn () , 2) ; let try_func_ty = bx . type_func (& [bx . type_ptr ()] , bx . type_void ()) ; bx . invoke (try_func_ty , None , None , try_func , & [data] , then , catch , None , None) ; bx . switch_to_block (then) ; bx . ret (bx . const_i32 (0)) ; bx . switch_to_block (catch) ; let lpad_ty = bx . type_struct (& [bx . type_ptr () , bx . type_i32 ()] , false) ; let vals = bx . landing_pad (lpad_ty , bx . eh_personality () , 1) ; let tydesc = bx . const_null (bx . type_ptr ()) ; bx . add_clause (vals , tydesc) ; let ptr = bx . extract_value (vals , 0) ; let catch_ty = bx . type_func (& [bx . type_ptr () , bx . type_ptr ()] , bx . type_void ()) ; bx . call (catch_ty , None , None , catch_func , & [data , ptr] , None , None) ; bx . ret (bx . const_i32 (1)) ; }) ; let ret = bx . call (llty , None , None , llfn , & [try_func , data , catch_func] , None , None) ; OperandValue :: Immediate (ret) . store (bx , dest) ; }
};
}
