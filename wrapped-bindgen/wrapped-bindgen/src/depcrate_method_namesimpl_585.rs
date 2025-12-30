// Generated macro for impl_585 (impl)
macro_rules! Depcrate_method_namesimpl_585 {
() => {
// Module: crate::method_names
// Provides: {"impl_585"}
// Dependencies: {}
impl MethodNames { pub fn new () -> Self { Self (HashMap :: new ()) } pub fn add (& mut self , method : MethodDef) -> TokenStream { let name = method_def_special_name (method) ; let overload = self . 0 . entry (name . to_string ()) . or_insert (0) ; * overload += 1 ; if * overload > 1 { format ! ("{name}{overload}") . into () } else { to_ident (& name) } } }
};
}
