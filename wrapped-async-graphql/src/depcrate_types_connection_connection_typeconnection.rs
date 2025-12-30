// Generated macro for Connection (struct)
macro_rules! Depcrate_types_connection_connection_typeConnection {
() => {
// Module: crate::types::connection::connection_type
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " Connection type"] # [doc = ""] # [doc = " Connection is the result of a query for `connection::query`."] pub struct Connection < Cursor , Node , ConnectionFields = EmptyFields , EdgeFields = EmptyFields , Name = DefaultConnectionName , EdgeName = DefaultEdgeName , NodesField = EnableNodesField , > where Cursor : CursorType + Send + Sync , Node : OutputType , ConnectionFields : ObjectType , EdgeFields : ObjectType , Name : ConnectionNameType , EdgeName : EdgeNameType , NodesField : NodesFieldSwitcherSealed , { _mark1 : PhantomData < Name > , _mark2 : PhantomData < EdgeName > , _mark3 : PhantomData < NodesField > , # [doc = " All edges of the current page."] pub edges : Vec < Edge < Cursor , Node , EdgeFields , EdgeName > > , # [doc = " Additional fields for connection object."] pub additional_fields : ConnectionFields , # [doc = " If `true` means has previous page."] pub has_previous_page : bool , # [doc = " If `true` means has next page."] pub has_next_page : bool , }
};
}
