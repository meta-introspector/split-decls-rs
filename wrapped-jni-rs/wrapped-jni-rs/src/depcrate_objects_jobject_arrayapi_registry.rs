// Generated macro for API_REGISTRY (static)
macro_rules! Depcrate_objects_jobject_arrayAPI_REGISTRY {
() => {
// Module: crate::objects::jobject_array
// Provides: {"API_REGISTRY"}
// Dependencies: {}
static API_REGISTRY : OnceLock < RwLock < HashMap < TypeId , & 'static (dyn Any + Send + Sync) > > > = OnceLock :: new () ;
};
}
