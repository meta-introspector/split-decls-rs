// Generated macro for impl_763 (impl)
macro_rules! Depcrate_struct_with_genericimpl_763 {
() => {
// Module: crate::struct_with_generic
// Provides: {"impl_763"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IReference < T > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({61c17706-2d65-11e0-9ae8-d48564015472}") . push_slice (b";") . push_other (T :: SIGNATURE) . push_slice (b")") ; }
};
}
