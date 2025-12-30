// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_types_connection_edgeimpl_1041 {
() => {
// Module: crate::types::connection::edge
// Provides: {"impl_1041"}
// Dependencies: {}
impl < Cursor , Node , EdgeFields , Name > TypeName for Edge < Cursor , Node , EdgeFields , Name > where Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , Name : EdgeNameType , { # [inline] fn type_name () -> Cow < 'static , str > { Name :: type_name :: < Node > () . into () } }
};
}
