// Generated macro for impl_226 (impl)
macro_rules! Depcrate_delegate_genericimpl_226 {
() => {
// Module: crate::delegate_generic
// Provides: {"impl_226"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for EventHandler < T > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f}") . push_slice (b";") . push_other (T :: SIGNATURE) . push_slice (b")") ; }
};
}
