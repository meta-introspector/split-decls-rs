// Generated macro for impl_35 (impl)
macro_rules! Depcrate_bindingsimpl_35 {
() => {
// Module: crate::bindings
// Provides: {"impl_35"}
// Dependencies: {}
impl IContainerVisualFactory_Vtbl { pub const fn new < Identity : IContainerVisualFactory_Impl , const OFFSET : isize > () -> Self { Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IContainerVisualFactory , OFFSET > () , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IContainerVisualFactory as windows_core :: Interface > :: IID } }
};
}
