// Generated macro for make_local_place (function)
macro_rules! Depcrate_abimake_local_place {
() => {
// Module: crate::abi
// Provides: {"make_local_place"}
// Dependencies: {}
# [doc = " Make a [`CPlace`] capable of holding value of the specified type."] fn make_local_place < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , local : Local , layout : TyAndLayout < 'tcx > , is_ssa : bool ,) -> CPlace < 'tcx > { if layout . is_unsized () { fx . tcx . dcx () . span_fatal (fx . mir . local_decls [local] . source_info . span , "unsized locals are not yet supported" ,) ; } let place = if is_ssa { if let BackendRepr :: ScalarPair (_ , _) = layout . backend_repr { CPlace :: new_var_pair (fx , local , layout) } else { CPlace :: new_var (fx , local , layout) } } else { CPlace :: new_stack_slot (fx , layout) } ; self :: comments :: add_local_place_comments (fx , place , local) ; place }
};
}
