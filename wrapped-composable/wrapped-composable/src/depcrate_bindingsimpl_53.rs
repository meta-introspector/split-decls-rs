// Generated macro for impl_53 (impl)
macro_rules! Depcrate_bindingsimpl_53 {
() => {
// Module: crate::bindings
// Provides: {"impl_53"}
// Dependencies: {}
impl IVisualFactory_Vtbl { pub const fn new < Identity : IVisualFactory_Impl , const OFFSET : isize > () -> Self { Self { base__ : windows_core :: IInspectable_Vtbl :: new :: < Identity , IVisualFactory , OFFSET > () , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IVisualFactory as windows_core :: Interface > :: IID } }
};
}
