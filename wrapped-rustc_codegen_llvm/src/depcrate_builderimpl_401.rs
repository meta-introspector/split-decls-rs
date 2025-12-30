// Generated macro for impl_401 (impl)
macro_rules! Depcrate_builderimpl_401 {
() => {
// Module: crate::builder
// Provides: {"impl_401"}
// Dependencies: {}
impl < 'a , 'll , 'tcx > Builder < 'a , 'll , 'tcx > { pub (crate) fn call_intrinsic (& mut self , base_name : impl Into < Cow < 'static , str > > , type_params : & [& 'll Type] , args : & [& 'll Value] ,) -> & 'll Value { let (ty , f) = self . cx . get_intrinsic (base_name . into () , type_params) ; self . call (ty , None , None , f , args , None , None) } fn call_lifetime_intrinsic (& mut self , intrinsic : & 'static str , ptr : & 'll Value , size : Size) { let size = size . bytes () ; if size == 0 { return ; } if ! self . cx () . sess () . emit_lifetime_markers () { return ; } if crate :: llvm_util :: get_version () >= (22 , 0 , 0) { self . call_intrinsic (intrinsic , & [self . val_ty (ptr)] , & [ptr]) ; } else { self . call_intrinsic (intrinsic , & [self . val_ty (ptr)] , & [self . cx . const_u64 (size) , ptr]) ; } } }
};
}
