// Generated macro for impl_1043 (impl)
macro_rules! Depcrate_types_connection_edgeimpl_1043 {
() => {
// Module: crate::types::connection::edge
// Provides: {"impl_1043"}
// Dependencies: {}
impl < Cursor , Node , Name > Edge < Cursor , Node , EmptyFields , Name > where Cursor : CursorType + Send + Sync , Node : OutputType , Name : EdgeNameType , { # [doc = " Create a new edge."] # [inline] pub fn new (cursor : Cursor , node : Node) -> Self { Self { _mark : PhantomData , cursor , node , additional_fields : EmptyFields , } } }
};
}
