// Generated macro for get_ptr_and_method_ref (function)
macro_rules! Depcrate_vtableget_ptr_and_method_ref {
() => {
// Module: crate::vtable
// Provides: {"get_ptr_and_method_ref"}
// Dependencies: {}
pub (crate) fn get_ptr_and_method_ref < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , mut arg : CValue < 'tcx > , idx : usize ,) -> (Pointer , Value) { if let BackendRepr :: Scalar (_) = arg . layout () . backend_repr { while ! arg . layout () . ty . is_raw_ptr () && ! arg . layout () . ty . is_ref () { let (idx , _) = arg . layout () . non_1zst_field (fx) . expect ("not exactly one non-1-ZST field in a `DispatchFromDyn` type") ; arg = arg . value_field (fx , idx) ; } } let (ptr , vtable) = if let BackendRepr :: ScalarPair (_ , _) = arg . layout () . backend_repr { let (ptr , vtable) = arg . load_scalar_pair (fx) ; (Pointer :: new (ptr) , vtable) } else { let (ptr , vtable) = arg . try_to_ptr () . unwrap () ; (ptr , vtable . unwrap ()) } ; let usize_size = fx . layout_of (fx . tcx . types . usize) . size . bytes () ; let func_ref = fx . bcx . ins () . load (fx . pointer_type , vtable_memflags () , vtable , (idx * usize_size as usize) as i32 ,) ; (ptr , func_ref) }
};
}
