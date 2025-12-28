macro_rules! deps {
    () => {
        CodegenCx!();
        UnnamedAddr!();
        CallConv!();
        SmallVec!();
        AttributePlace!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < 'll , 'tcx > MiscCodegenMethods < 'tcx > for CodegenCx < 'll , 'tcx > { fn vtables (& self ,) -> & RefCell < FxHashMap < (Ty < 'tcx > , Option < ty :: ExistentialTraitRef < 'tcx > >) , & 'll Value > > { & self . vtables } fn apply_vcall_visibility_metadata (& self , ty : Ty < 'tcx > , poly_trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > , vtable : & 'll Value ,) { apply_vcall_visibility_metadata (self , ty , poly_trait_ref , vtable) ; } fn get_fn (& self , instance : Instance < 'tcx >) -> & 'll Value { get_fn (self , instance) } fn get_fn_addr (& self , instance : Instance < 'tcx >) -> & 'll Value { get_fn (self , instance) } fn eh_personality (& self) -> & 'll Value { if let Some (llpersonality) = self . eh_personality . get () { return llpersonality ; } let name = if wants_msvc_seh (self . sess ()) { Some ("__CxxFrameHandler3") } else if wants_wasm_eh (self . sess ()) { Some ("__gxx_wasm_personality_v0") } else { None } ; let tcx = self . tcx ; let llfn = match tcx . lang_items () . eh_personality () { Some (def_id) if name . is_none () => self . get_fn_addr (ty :: Instance :: expect_resolve (tcx , self . typing_env () , def_id , ty :: List :: empty () , DUMMY_SP ,)) , _ => { let name = name . unwrap_or ("rust_eh_personality") ; if let Some (llfn) = self . get_declared_value (name) { llfn } else { let fty = self . type_variadic_func (& [] , self . type_i32 ()) ; let llfn = self . declare_cfn (name , llvm :: UnnamedAddr :: Global , fty) ; let target_cpu = attributes :: target_cpu_attr (self) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & [target_cpu]) ; llfn } } } ; self . eh_personality . set (Some (llfn)) ; llfn } fn sess (& self) -> & Session { self . tcx . sess } fn set_frame_pointer_type (& self , llfn : & 'll Value) { if let Some (attr) = attributes :: frame_pointer_type_attr (self) { attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & [attr]) ; } } fn apply_target_cpu_attr (& self , llfn : & 'll Value) { let mut attrs = SmallVec :: < [_ ; 2] > :: new () ; attrs . push (attributes :: target_cpu_attr (self)) ; attrs . extend (attributes :: tune_cpu_attr (self)) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , & attrs) ; } fn declare_c_main (& self , fn_type : Self :: Type) -> Option < Self :: Function > { let entry_name = self . sess () . target . entry_name . as_ref () ; if self . get_declared_value (entry_name) . is_none () { let llfn = self . declare_entry_fn (entry_name , llvm :: CallConv :: from_conv (self . sess () . target . entry_abi , self . sess () . target . arch . borrow () ,) , llvm :: UnnamedAddr :: Global , fn_type ,) ; attributes :: apply_to_llfn (llfn , llvm :: AttributePlace :: Function , attributes :: target_features_attr (self , vec ! []) . as_slice () ,) ; Some (llfn) } else { None } } }
    };
}

impl_222!();