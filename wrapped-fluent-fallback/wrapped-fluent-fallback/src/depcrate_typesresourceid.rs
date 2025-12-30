// Generated macro for ResourceId (struct)
macro_rules! Depcrate_typesResourceId {
() => {
// Module: crate::types
// Provides: {"ResourceId"}
// Dependencies: {}
# [doc = " A resource identifier for a localization resource."] # [derive (Clone , Debug)] pub struct ResourceId { # [doc = " The resource identifier."] pub value : String , # [doc = " The [`ResourceType`] for this resource."] # [doc = ""] # [doc = " The default value (when converting from another type) is"] # [doc = " [`ResourceType::Required`]. You should only set this to"] # [doc = " [`ResourceType::Optional`] for experimental or under-development"] # [doc = " features that may not yet have content in all eventually-supported locales."] # [doc = ""] # [doc = " Setting this value to [`ResourceType::Optional`] for all resources"] # [doc = " may have a severe impact on performance due to increasing the state space"] # [doc = " of the solver."] pub resource_type : ResourceType , }
};
}
