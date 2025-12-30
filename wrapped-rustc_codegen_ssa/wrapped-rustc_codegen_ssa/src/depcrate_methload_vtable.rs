// Generated macro for load_vtable (function)
macro_rules! Depcrate_methload_vtable {
() => {
// Module: crate::meth
// Provides: {"load_vtable"}
// Dependencies: {}
# [doc = " Call this function whenever you need to load a vtable."] pub (crate) fn load_vtable < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , llvtable : Bx :: Value , llty : Bx :: Type , vtable_byte_offset : u64 , ty : Ty < 'tcx > , nonnull : bool ,) -> Bx :: Value { let ptr_align = bx . data_layout () . pointer_align () . abi ; if bx . cx () . sess () . opts . unstable_opts . virtual_function_elimination && bx . cx () . sess () . lto () == Lto :: Fat { if let Some (trait_ref) = dyn_trait_in_self (bx . tcx () , ty) { let typeid = bx . typeid_metadata (typeid_for_trait_ref (bx . tcx () , trait_ref) . as_bytes ()) . unwrap () ; let func = bx . type_checked_load (llvtable , vtable_byte_offset , typeid) ; return func ; } else if nonnull { bug ! ("load nonnull value from a vtable without a principal trait") } } let gep = bx . inbounds_ptradd (llvtable , bx . const_usize (vtable_byte_offset)) ; let ptr = bx . load (llty , gep , ptr_align) ; bx . set_invariant_load (ptr) ; if nonnull { bx . nonnull_metadata (ptr) ; } ptr }
};
}
