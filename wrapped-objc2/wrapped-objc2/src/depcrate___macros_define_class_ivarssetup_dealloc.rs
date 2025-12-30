// Generated macro for setup_dealloc (function)
macro_rules! Depcrate___macros_define_class_ivarssetup_dealloc {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"setup_dealloc"}
// Dependencies: {}
pub (crate) fn setup_dealloc < T : DefinedClass > (builder : & mut ClassBuilder) where T :: Super : ClassType , { if mem :: needs_drop :: < T > () || mem :: needs_drop :: < T :: Ivars > () { let func : unsafe extern "C-unwind" fn (_ , _) = dealloc :: < T > ; unsafe { builder . add_method (sel ! (dealloc) , func) } ; } else { } }
};
}
