mkuse!{use crate :: constant :: data_id_for_vtable ;}
mkuse!{use crate :: prelude :: * ;}

macro_rules! vtable_memflags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vtable_memflags in module {}", module_path!());
    };
}

mkfn!{
    vtable_memflags_introspect!();
    pub (crate) fn vtable_memflags () -> MemFlags { let mut flags = MemFlags :: trusted () ; flags . set_readonly () ; flags }
}

macro_rules! drop_fn_of_obj_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function drop_fn_of_obj in module {}", module_path!());
    };
}

mkfn!{
    drop_fn_of_obj_introspect!();
    pub (crate) fn drop_fn_of_obj (fx : & mut FunctionCx < '_ , '_ , '_ > , vtable : Value) -> Value { let usize_size = fx . layout_of (fx . tcx . types . usize) . size . bytes () as usize ; fx . bcx . ins () . load (fx . pointer_type , vtable_memflags () , vtable , (ty :: COMMON_VTABLE_ENTRIES_DROPINPLACE * usize_size) as i32 ,) }
}

macro_rules! size_of_obj_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function size_of_obj in module {}", module_path!());
    };
}

mkfn!{
    size_of_obj_introspect!();
    pub (crate) fn size_of_obj (fx : & mut FunctionCx < '_ , '_ , '_ > , vtable : Value) -> Value { let usize_size = fx . layout_of (fx . tcx . types . usize) . size . bytes () as usize ; fx . bcx . ins () . load (fx . pointer_type , vtable_memflags () , vtable , (ty :: COMMON_VTABLE_ENTRIES_SIZE * usize_size) as i32 ,) }
}

macro_rules! align_of_obj_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function align_of_obj in module {}", module_path!());
    };
}

mkfn!{
    align_of_obj_introspect!();
    pub (crate) fn align_of_obj (fx : & mut FunctionCx < '_ , '_ , '_ > , vtable : Value) -> Value { let usize_size = fx . layout_of (fx . tcx . types . usize) . size . bytes () as usize ; fx . bcx . ins () . load (fx . pointer_type , vtable_memflags () , vtable , (ty :: COMMON_VTABLE_ENTRIES_ALIGN * usize_size) as i32 ,) }
}

macro_rules! get_ptr_and_method_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_ptr_and_method_ref in module {}", module_path!());
    };
}

mkfn!{
    get_ptr_and_method_ref_introspect!();
    pub (crate) fn get_ptr_and_method_ref < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , mut arg : CValue < 'tcx > , idx : usize ,) -> (Pointer , Value) { if let BackendRepr :: Scalar (_) = arg . layout () . backend_repr { while ! arg . layout () . ty . is_raw_ptr () && ! arg . layout () . ty . is_ref () { let (idx , _) = arg . layout () . non_1zst_field (fx) . expect ("not exactly one non-1-ZST field in a `DispatchFromDyn` type") ; arg = arg . value_field (fx , idx) ; } } let (ptr , vtable) = if let BackendRepr :: ScalarPair (_ , _) = arg . layout () . backend_repr { let (ptr , vtable) = arg . load_scalar_pair (fx) ; (Pointer :: new (ptr) , vtable) } else { let (ptr , vtable) = arg . try_to_ptr () . unwrap () ; (ptr , vtable . unwrap ()) } ; let usize_size = fx . layout_of (fx . tcx . types . usize) . size . bytes () ; let func_ref = fx . bcx . ins () . load (fx . pointer_type , vtable_memflags () , vtable , (idx * usize_size as usize) as i32 ,) ; (ptr , func_ref) }
}

macro_rules! get_vtable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_vtable in module {}", module_path!());
    };
}

mkfn!{
    get_vtable_introspect!();
    pub (crate) fn get_vtable < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , ty : Ty < 'tcx > , trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > ,) -> Value { let data_id = data_id_for_vtable (fx . tcx , & mut fx . constants_cx , fx . module , ty , trait_ref) ; let local_data_id = fx . module . declare_data_in_func (data_id , fx . bcx . func) ; if fx . clif_comments . enabled () { fx . add_comment (local_data_id , "vtable") ; } fx . bcx . ins () . global_value (fx . pointer_type , local_data_id) }
}