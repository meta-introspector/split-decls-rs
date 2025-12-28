macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! unsized_info {
    () => {
        deps!();
        # [doc = " Retrieves the information we are losing (making dynamic) in an unsizing"] # [doc = " adjustment."] # [doc = ""] # [doc = " The `old_info` argument is a bit odd. It is intended for use in an upcast,"] # [doc = " where the new vtable for an object will be derived from the old one."] fn unsized_info < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , source : Ty < 'tcx > , target : Ty < 'tcx > , old_info : Option < Bx :: Value > ,) -> Bx :: Value { let cx = bx . cx () ; let (source , target) = cx . tcx () . struct_lockstep_tails_for_codegen (source , target , bx . typing_env ()) ; match (source . kind () , target . kind ()) { (& ty :: Array (_ , len) , & ty :: Slice (_)) => cx . const_usize (len . try_to_target_usize (cx . tcx ()) . expect ("expected monomorphic const in codegen") ,) , (& ty :: Dynamic (data_a , _ , src_dyn_kind) , & ty :: Dynamic (data_b , _ , target_dyn_kind)) if src_dyn_kind == target_dyn_kind => { let old_info = old_info . expect ("unsized_info: missing old info for trait upcasting coercion") ; let b_principal_def_id = data_b . principal_def_id () ; if data_a . principal_def_id () == b_principal_def_id || b_principal_def_id . is_none () { debug_assert ! (validate_trivial_unsize (cx . tcx () , data_a , data_b) , "NOP unsize vtable changed principal trait ref: {data_a} -> {data_b}") ; return old_info ; } let vptr_entry_idx = cx . tcx () . supertrait_vtable_slot ((source , target)) ; if let Some (entry_idx) = vptr_entry_idx { let ptr_size = bx . data_layout () . pointer_size () ; let vtable_byte_offset = u64 :: try_from (entry_idx) . unwrap () * ptr_size . bytes () ; load_vtable (bx , old_info , bx . type_ptr () , vtable_byte_offset , source , true) } else { old_info } } (_ , ty :: Dynamic (data , _ , _)) => meth :: get_vtable (cx , source , data . principal () . map (| principal | bx . tcx () . instantiate_bound_regions_with_erased (principal)) ,) , _ => bug ! ("unsized_info: invalid unsizing {:?} -> {:?}" , source , target) , } }
    };
}

unsized_info!();