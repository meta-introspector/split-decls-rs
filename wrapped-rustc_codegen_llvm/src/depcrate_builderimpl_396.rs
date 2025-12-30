// Generated macro for impl_396 (impl)
macro_rules! Depcrate_builderimpl_396 {
() => {
// Module: crate::builder
// Provides: {"impl_396"}
// Dependencies: {}
impl < 'll > StaticBuilderMethods for Builder < '_ , 'll , '_ > { fn get_static (& mut self , def_id : DefId) -> & 'll Value { let global = self . cx () . get_static (def_id) ; if self . cx () . tcx . is_thread_local_static (def_id) { let pointer = self . call_intrinsic ("llvm.threadlocal.address" , & [self . val_ty (global)] , & [global]) ; self . pointercast (pointer , self . type_ptr ()) } else { self . cx () . const_pointercast (global , self . type_ptr ()) } } }
};
}
