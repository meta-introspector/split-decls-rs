// Generated macro for impl_457 (impl)
macro_rules! Depcrate_interface_iterableimpl_457 {
() => {
// Module: crate::interface_iterable
// Provides: {"impl_457"}
// Dependencies: {}
impl < T : windows_core :: RuntimeType + 'static > windows_core :: RuntimeType for IIterable < T > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({faa585ea-6214-4217-afda-7f46de5869b3}") . push_slice (b";") . push_other (T :: SIGNATURE) . push_slice (b")") ; }
};
}
