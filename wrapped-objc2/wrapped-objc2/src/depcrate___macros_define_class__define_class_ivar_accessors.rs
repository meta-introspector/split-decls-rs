// Generated macro for __define_class_ivar_accessors (macro)
macro_rules! Depcrate___macros_define_class__define_class_ivar_accessors {
() => {
// Module: crate::__macros::define_class
// Provides: {"__define_class_ivar_accessors"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_ivar_accessors { (($ class : ident) $ (,) ?) => { } ; (($ class : ident) $ ($ (# [$ ($ ivar_attrs : tt) *]) * $ ivar_vis : vis $ ivar : ident : $ ivar_ty : ty) ,+ $ (,) ?) => { # [doc = " Instance variable accessors."] impl $ class { $ ($ (# [$ ($ ivar_attrs) *]) * $ ivar_vis fn $ ivar (& self) -> &$ ivar_ty { # [allow (deprecated)] &< Self as $ crate :: DefinedClass >:: __get_ivars (self) .$ ivar }) + } } ; (($ class : ident) $ ($ ivars : tt) *) => { $ crate :: __macros :: compile_error ! ($ crate :: __macros :: concat ! ("invalid ivars\n" , $ crate :: __macros :: stringify ! ($ ($ ivars) *) ,)) ; } }
};
}
