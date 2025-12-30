// Generated macro for impl_1040 (impl)
macro_rules! Depcrate_types_connection_edgeimpl_1040 {
() => {
// Module: crate::types::connection::edge
// Provides: {"impl_1040"}
// Dependencies: {}
# [ComplexObject (internal)] impl < Cursor , Node , EdgeFields , Name > Edge < Cursor , Node , EdgeFields , Name > where Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , Name : EdgeNameType , { # [doc = " A cursor for use in pagination"] async fn cursor (& self) -> String { self . cursor . encode_cursor () } }
};
}
