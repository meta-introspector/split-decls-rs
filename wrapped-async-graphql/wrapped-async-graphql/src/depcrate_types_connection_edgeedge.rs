// Generated macro for Edge (struct)
macro_rules! Depcrate_types_connection_edgeEdge {
() => {
// Module: crate::types::connection::edge
// Provides: {"Edge"}
// Dependencies: {}
# [doc = " An edge in a connection."] # [derive (SimpleObject)] # [graphql (internal , name_type , shareable , complex)] pub struct Edge < Cursor , Node , EdgeFields , Name = DefaultEdgeName > where Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , Name : EdgeNameType , { # [graphql (skip)] _mark : PhantomData < Name > , # [doc = " A cursor for use in pagination"] # [graphql (skip)] pub cursor : Cursor , # [doc = " The item at the end of the edge"] pub node : Node , # [graphql (flatten)] pub (crate) additional_fields : EdgeFields , }
};
}
