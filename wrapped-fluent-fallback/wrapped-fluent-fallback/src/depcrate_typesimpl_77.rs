// Generated macro for impl_77 (impl)
macro_rules! Depcrate_typesimpl_77 {
() => {
// Module: crate::types
// Provides: {"impl_77"}
// Dependencies: {}
impl ResourceId { pub fn new < S : Into < String > > (value : S , resource_type : ResourceType) -> Self { Self { value : value . into () , resource_type , } } # [doc = " Returns [`true`] if the resource has [`ResourceType::Required`],"] # [doc = " otherwise returns [`false`]."] pub fn is_required (& self) -> bool { matches ! (self . resource_type , ResourceType :: Required) } # [doc = " Returns [`true`] if the resource has [`ResourceType::Optional`],"] # [doc = " otherwise returns [`false`]."] pub fn is_optional (& self) -> bool { matches ! (self . resource_type , ResourceType :: Optional) } }
};
}
