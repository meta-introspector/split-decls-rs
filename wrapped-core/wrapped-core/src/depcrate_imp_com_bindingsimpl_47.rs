// Generated macro for impl_47 (impl)
macro_rules! Depcrate_imp_com_bindingsimpl_47 {
() => {
// Module: crate::imp::com_bindings
// Provides: {"impl_47"}
// Dependencies: {}
impl IAgileObject_Vtbl { pub const fn new < Identity : IAgileObject_Impl , const OFFSET : isize > () -> Self { Self { base__ : windows_core :: IUnknown_Vtbl :: new :: < Identity , OFFSET > () , } } pub fn matches (iid : & windows_core :: GUID) -> bool { iid == & < IAgileObject as windows_core :: Interface > :: IID } }
};
}
