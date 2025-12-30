// Generated macro for alloc_environ_block (function)
macro_rules! Depcrate_shims_unix_envalloc_environ_block {
() => {
// Module: crate::shims::unix::env
// Provides: {"alloc_environ_block"}
// Dependencies: {}
# [doc = " Allocates an `environ` block with the given list of pointers."] fn alloc_environ_block < 'tcx > (ecx : & mut InterpCx < 'tcx , MiriMachine < 'tcx > > , mut vars : IndexVec < FieldIdx , Pointer > ,) -> InterpResult < 'tcx , Pointer > { vars . push (Pointer :: null ()) ; let vars_layout = ecx . layout_of (Ty :: new_array (* ecx . tcx , ecx . machine . layouts . mut_raw_ptr . ty , u64 :: try_from (vars . len ()) . unwrap () ,)) ? ; let vars_place = ecx . allocate (vars_layout , MiriMemoryKind :: Machine . into ()) ? ; for (idx , var) in vars . into_iter_enumerated () { let place = ecx . project_field (& vars_place , idx) ? ; ecx . write_pointer (var , & place) ? ; } interp_ok (vars_place . ptr ()) }
};
}
