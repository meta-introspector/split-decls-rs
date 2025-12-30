// Generated macro for impl_1539 (impl)
macro_rules! Depcrate_registryimpl_1539 {
() => {
// Module: crate::registry
// Provides: {"impl_1539"}
// Dependencies: {}
impl MetaDirectiveInvocation { pub fn sdl (& self) -> String { let formatted_args = if self . args . is_empty () { String :: new () } else { format ! ("({})" , self . args . iter () . map (| (name , value) | format ! ("{}: {}" , name , value)) . collect ::< Vec < _ >> () . join (", ")) } ; format ! ("@{}{}" , self . name , formatted_args) } }
};
}
