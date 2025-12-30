// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_read_pe_resourceimpl_1051 {
() => {
// Module: crate::read::pe::resource
// Provides: {"impl_1051"}
// Dependencies: {}
impl ResourceNameOrId { # [doc = " Converts to an option of name."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn name (self) -> Option < ResourceName > { match self { Self :: Name (name) => Some (name) , _ => None , } } # [doc = " Converts to an option of ID."] # [doc = ""] # [doc = " Helper for iterator filtering."] pub fn id (self) -> Option < u16 > { match self { Self :: Id (id) => Some (id) , _ => None , } } }
};
}
