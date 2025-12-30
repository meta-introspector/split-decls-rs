// Generated macro for impl_1013 (impl)
macro_rules! Depcrate_types_connection_connection_typeimpl_1013 {
() => {
// Module: crate::types::connection::connection_type
// Provides: {"impl_1013"}
// Dependencies: {}
impl < Cursor , Node , NodesField , ConnectionFields , EdgeFields , Name , EdgeName > Connection < Cursor , Node , ConnectionFields , EdgeFields , Name , EdgeName , NodesField > where Cursor : CursorType + Send + Sync , Node : OutputType , ConnectionFields : ObjectType , EdgeFields : ObjectType , Name : ConnectionNameType , EdgeName : EdgeNameType , NodesField : NodesFieldSwitcherSealed , { # [doc = " Create a new connection, it can have some additional fields."] # [inline] pub fn with_additional_fields (has_previous_page : bool , has_next_page : bool , additional_fields : ConnectionFields ,) -> Self { Connection { _mark1 : PhantomData , _mark2 : PhantomData , _mark3 : PhantomData , additional_fields , has_previous_page , has_next_page , edges : Vec :: new () , } } }
};
}
