// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_types_connection_connection_typeimpl_1015 {
() => {
// Module: crate::types::connection::connection_type
// Provides: {"impl_1015"}
// Dependencies: {}
# [Object (internal , name_type , shareable)] impl < Cursor , Node , ConnectionFields , EdgeFields , Name , EdgeName > Connection < Cursor , Node , ConnectionFields , EdgeFields , Name , EdgeName , EnableNodesField > where Cursor : CursorType + Send + Sync , Node : OutputType , ConnectionFields : ObjectType , EdgeFields : ObjectType , Name : ConnectionNameType , EdgeName : EdgeNameType , { # [doc = " Information to aid in pagination."] async fn page_info (& self) -> PageInfo { PageInfo { has_previous_page : self . has_previous_page , has_next_page : self . has_next_page , start_cursor : self . edges . first () . map (| edge | edge . cursor . encode_cursor ()) , end_cursor : self . edges . last () . map (| edge | edge . cursor . encode_cursor ()) , } } # [doc = " A list of edges."] # [inline] async fn edges (& self) -> & [Edge < Cursor , Node , EdgeFields , EdgeName >] { & self . edges } # [doc = " A list of nodes."] async fn nodes (& self) -> Vec < & Node > { self . edges . iter () . map (| e | & e . node) . collect () } # [graphql (flatten)] # [inline] async fn additional_fields (& self) -> & ConnectionFields { & self . additional_fields } }
};
}
