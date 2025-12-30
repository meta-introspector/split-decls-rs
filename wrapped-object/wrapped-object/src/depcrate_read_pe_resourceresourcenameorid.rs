// Generated macro for ResourceNameOrId (enum)
macro_rules! Depcrate_read_pe_resourceResourceNameOrId {
() => {
// Module: crate::read::pe::resource
// Provides: {"ResourceNameOrId"}
// Dependencies: {}
# [doc = " A resource name or ID."] # [doc = ""] # [doc = " Can be either a string or a numeric ID."] # [derive (Debug)] pub enum ResourceNameOrId { # [doc = " A resource name."] Name (ResourceName) , # [doc = " A resource ID."] Id (u16) , }
};
}
