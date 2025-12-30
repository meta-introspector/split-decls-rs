// Generated macro for ResourceType (enum)
macro_rules! Depcrate_typesResourceType {
() => {
// Module: crate::types
// Provides: {"ResourceType"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub enum ResourceType { # [doc = " This is a required resource."] # [doc = ""] # [doc = " A bundle generator should not consider a solution as valid"] # [doc = " if this resource is missing."] # [doc = ""] # [doc = " This is the default when creating a [`ResourceId`]."] Required , # [doc = " This is an optional resource."] # [doc = ""] # [doc = " A bundle generator should still populate a partial solution"] # [doc = " even if this resource is missing."] # [doc = ""] # [doc = " This is intended for experimental and/or under-development"] # [doc = " resources that may not have content for all supported locales."] # [doc = ""] # [doc = " This should be used sparingly, as it will greatly increase"] # [doc = " the state space of the search for valid solutions which can"] # [doc = " have a severe impact on performance."] Optional , }
};
}
