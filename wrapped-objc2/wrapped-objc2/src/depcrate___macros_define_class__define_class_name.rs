// Generated macro for __define_class_name (macro)
macro_rules! Depcrate___macros_define_class__define_class_name {
() => {
// Module: crate::__macros::define_class
// Provides: {"__define_class_name"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_name { ($ class : ident , $ ($ name : tt) +) => { $ ($ name) + } ; ($ class : ident ,) => { $ crate :: __macros :: concat ! ($ crate :: __macros :: module_path ! () , "::" , $ crate :: __macros :: stringify ! ($ class) , $ crate :: __macros :: env ! ("CARGO_PKG_VERSION") ,) } ; }
};
}
