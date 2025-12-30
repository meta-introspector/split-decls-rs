// Generated macro for ToResourceId (trait)
macro_rules! Depcrate_typesToResourceId {
() => {
// Module: crate::types
// Provides: {"ToResourceId"}
// Dependencies: {}
# [doc = " A trait for creating a [`ResourceId`] from another type."] # [doc = ""] # [doc = " This differs from the [`From`] trait in that the [`From`] trait"] # [doc = " always takes the default resource type of [`ResourceType::Required`]."] # [doc = ""] # [doc = " If you need to create a resource with a non-default [`ResourceType`],"] # [doc = " such as [`ResourceType::Optional`], then use this trait."] # [doc = ""] # [doc = " This trait is automatically implemented for types that implement [`Into<String>`]."] pub trait ToResourceId { # [doc = " Creates a [`ResourceId`] from [`self`], given a [`ResourceType`]."] fn to_resource_id (self , resource_type : ResourceType) -> ResourceId ; }
};
}
