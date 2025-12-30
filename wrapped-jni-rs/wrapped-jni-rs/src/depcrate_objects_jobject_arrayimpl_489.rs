// Generated macro for impl_489 (impl)
macro_rules! Depcrate_objects_jobject_arrayimpl_489 {
() => {
// Module: crate::objects::jobject_array
// Provides: {"impl_489"}
// Dependencies: {}
impl < 'local , E : Reference > :: std :: ops :: Deref for JObjectArray < 'local , E > { type Target = JObject < 'local > ; fn deref (& self) -> & Self :: Target { & self . array } }
};
}
