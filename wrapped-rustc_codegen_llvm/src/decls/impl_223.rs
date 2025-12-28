macro_rules! deps {
    () => {
        UnnamedAddr!();
        Intrinsic!();
        SmallVec!();
        CodegenCx!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < 'll > CodegenCx < 'll , '_ > { pub (crate) fn get_intrinsic (& self , base_name : Cow < 'static , str > , type_params : & [& 'll Type] ,) -> (& 'll Type , & 'll Value) { * self . intrinsics . borrow_mut () . entry ((base_name , SmallVec :: from_slice (type_params))) . or_insert_with_key (| (base_name , type_params) | { self . declare_intrinsic (base_name , type_params) }) } fn declare_intrinsic (& self , base_name : & str , type_params : & [& 'll Type] ,) -> (& 'll Type , & 'll Value) { if base_name == "memcmp" { let fn_ty = self . type_func (& [self . type_ptr () , self . type_ptr () , self . type_isize ()] , self . type_int ()) ; let f = self . declare_cfn ("memcmp" , llvm :: UnnamedAddr :: No , fn_ty) ; return (fn_ty , f) ; } let intrinsic = llvm :: Intrinsic :: lookup (base_name . as_bytes ()) . unwrap_or_else (| | bug ! ("Unknown intrinsic: `{base_name}`")) ; let f = intrinsic . get_declaration (self . llmod , & type_params) ; (self . get_type_of_global (f) , f) } pub (crate) fn eh_catch_typeinfo (& self) -> & 'll Value { if let Some (eh_catch_typeinfo) = self . eh_catch_typeinfo . get () { return eh_catch_typeinfo ; } let tcx = self . tcx ; assert ! (self . sess () . target . os == "emscripten") ; let eh_catch_typeinfo = match tcx . lang_items () . eh_catch_typeinfo () { Some (def_id) => self . get_static (def_id) , _ => { let ty = self . type_struct (& [self . type_ptr () , self . type_ptr ()] , false) ; self . declare_global (& mangle_internal_symbol (self . tcx , "rust_eh_catch_typeinfo") , ty) } } ; self . eh_catch_typeinfo . set (Some (eh_catch_typeinfo)) ; eh_catch_typeinfo } }
    };
}

impl_223!()