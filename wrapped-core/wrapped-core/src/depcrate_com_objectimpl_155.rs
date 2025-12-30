// Generated macro for impl_155 (impl)
macro_rules! Depcrate_com_objectimpl_155 {
() => {
// Module: crate::com_object
// Provides: {"impl_155"}
// Dependencies: {}
impl < T > core :: ops :: Deref for StaticComObject < T > where T : ComObjectInner , { type Target = T :: Outer ; fn deref (& self) -> & Self :: Target { & self . outer } }
};
}
