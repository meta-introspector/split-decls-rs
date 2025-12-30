// Generated macro for impl_1042 (impl)
macro_rules! Depcrate_types_connection_edgeimpl_1042 {
() => {
// Module: crate::types::connection::edge
// Provides: {"impl_1042"}
// Dependencies: {}
impl < Cursor , Node , EdgeFields , Name > Edge < Cursor , Node , EdgeFields , Name > where Name : EdgeNameType , Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , { # [doc = " Create a new edge, it can have some additional fields."] # [inline] pub fn with_additional_fields (cursor : Cursor , node : Node , additional_fields : EdgeFields ,) -> Self { Self { _mark : PhantomData , cursor , node , additional_fields , } } }
};
}
