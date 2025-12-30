// Generated macro for impl_1016 (impl)
macro_rules! Depcrate_types_connection_connection_typeimpl_1016 {
() => {
// Module: crate::types::connection::connection_type
// Provides: {"impl_1016"}
// Dependencies: {}
impl < Cursor , Node , ConnectionFields , EdgeFields , Name , EdgeName , NodesField > TypeName for Connection < Cursor , Node , ConnectionFields , EdgeFields , Name , EdgeName , NodesField > where Cursor : CursorType + Send + Sync , Node : OutputType , ConnectionFields : ObjectType , EdgeFields : ObjectType , Name : ConnectionNameType , EdgeName : EdgeNameType , NodesField : NodesFieldSwitcherSealed , { # [inline] fn type_name () -> Cow < 'static , str > { Name :: type_name :: < Node > () . into () } }
};
}
