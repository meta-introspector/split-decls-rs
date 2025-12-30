// Generated macro for impl_527 (impl)
macro_rules! Depcrate_type_ofimpl_527 {
() => {
// Module: crate::type_of
// Provides: {"impl_527"}
// Dependencies: {}
impl < 'gcc , 'tcx > LayoutTypeCodegenMethods < 'tcx > for CodegenCx < 'gcc , 'tcx > { fn backend_type (& self , layout : TyAndLayout < 'tcx >) -> Type < 'gcc > { layout . gcc_type (self) } fn immediate_backend_type (& self , layout : TyAndLayout < 'tcx >) -> Type < 'gcc > { layout . immediate_gcc_type (self) } fn is_backend_immediate (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_gcc_immediate () } fn is_backend_scalar_pair (& self , layout : TyAndLayout < 'tcx >) -> bool { layout . is_gcc_scalar_pair () } fn scalar_pair_element_backend_type (& self , layout : TyAndLayout < 'tcx > , index : usize , _immediate : bool ,) -> Type < 'gcc > { layout . scalar_pair_element_gcc_type (self , index) } fn cast_backend_type (& self , ty : & CastTarget) -> Type < 'gcc > { ty . gcc_type (self) } fn fn_ptr_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> Type < 'gcc > { fn_abi . ptr_to_gcc_type (self) } fn reg_backend_type (& self , _ty : & Reg) -> Type < 'gcc > { unimplemented ! () ; } fn fn_decl_backend_type (& self , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > >) -> Type < 'gcc > { let FnAbiGcc { return_type , arguments_type , is_c_variadic , .. } = fn_abi . gcc_type (self) ; self . context . new_function_pointer_type (None , return_type , & arguments_type , is_c_variadic) } }
};
}
