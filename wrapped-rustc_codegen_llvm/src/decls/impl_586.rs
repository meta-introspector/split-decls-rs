macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl < 'll , 'tcx > LayoutTypeCodegenMethods < 'tcx > for CodegenCx < 'll , 'tcx > { fn backend_type (& self , layout : TyAndLayout < 'tcx >) -> & 'll Type { layout . llvm_type (self) } fn immediate_backend_type (& self , layout : TyAndLayout < 'tcx >) -> & 'll Type { layout . immediate_llvm_type (self) } fn is_backend_immediate (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_llvm_immediate () } fn is_backend_scalar_pair (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_llvm_scalar_pair () } fn scalar_pair_element_backend_type (& self , layout : TyAndLayout < 'tcx > , index : usize , immediate : bool ,) -> & 'll Type { layout . scalar_pair_element_llvm_type (self , index , immediate) } fn cast_backend_type (& self , ty : & CastTarget) -> & 'll Type { ty . llvm_type (self) } fn fn_decl_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> & 'll Type { fn_abi . llvm_type (self) } fn fn_ptr_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> & 'll Type { fn_abi . ptr_to_llvm_type (self) } fn reg_backend_type (& self , ty : & Reg) -> & 'll Type { ty . llvm_type (self) } }
    };
}

impl_586!()