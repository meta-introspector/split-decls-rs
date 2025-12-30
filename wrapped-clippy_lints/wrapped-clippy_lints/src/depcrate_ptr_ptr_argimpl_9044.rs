// Generated macro for impl_9044 (impl)
macro_rules! Depcrate_ptr_ptr_argimpl_9044 {
() => {
// Module: crate::ptr::ptr_arg
// Provides: {"impl_9044"}
// Dependencies: {}
impl < 'tcx > DerefTy < 'tcx > { fn ty (& self , cx : & LateContext < 'tcx >) -> Ty < 'tcx > { match * self { Self :: Str => cx . tcx . types . str_ , Self :: Path => Ty :: new_adt (cx . tcx , cx . tcx . adt_def (cx . tcx . get_diagnostic_item (sym :: Path) . unwrap ()) , List :: empty () ,) , Self :: Slice (_ , ty) => Ty :: new_slice (cx . tcx , ty) , } } fn argless_str (& self) -> & 'static str { match * self { Self :: Str => "str" , Self :: Path => "Path" , Self :: Slice (..) => "[_]" , } } fn display < 'a > (& 'a self , cx : & 'a LateContext < 'tcx >) -> DerefTyDisplay < 'a , 'tcx > { DerefTyDisplay (cx , self) } }
};
}
