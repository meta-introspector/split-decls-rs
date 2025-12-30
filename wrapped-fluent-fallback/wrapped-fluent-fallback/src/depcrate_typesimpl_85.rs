// Generated macro for impl_85 (impl)
macro_rules! Depcrate_typesimpl_85 {
() => {
// Module: crate::types
// Provides: {"impl_85"}
// Dependencies: {}
impl < S : Into < String > > ToResourceId for S { fn to_resource_id (self , resource_type : ResourceType) -> ResourceId { ResourceId :: new (self . into () , resource_type) } }
};
}
