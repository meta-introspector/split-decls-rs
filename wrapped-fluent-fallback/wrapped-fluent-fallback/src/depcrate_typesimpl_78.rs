// Generated macro for impl_78 (impl)
macro_rules! Depcrate_typesimpl_78 {
() => {
// Module: crate::types
// Provides: {"impl_78"}
// Dependencies: {}
impl < S : Into < String > > From < S > for ResourceId { fn from (id : S) -> Self { Self { value : id . into () , resource_type : ResourceType :: Required , } } }
};
}
