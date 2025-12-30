// Generated macro for get_operation (function)
macro_rules! Depcrate_executorget_operation {
() => {
// Module: crate::executor
// Provides: {"get_operation"}
// Dependencies: {}
# [doc (hidden)] pub fn get_operation < 'b , 'd , S > (document : & 'b Document < 'd , S > , operation_name : Option < & str > ,) -> Result < & 'b Spanning < Operation < 'd , S > > , GraphQLError > where S : ScalarValue , { let mut operation = None ; for def in document { if let Definition :: Operation (op) = def { if operation_name . is_none () && operation . is_some () { return Err (GraphQLError :: MultipleOperationsProvided) ; } let move_op = operation_name . is_none () || op . item . name . as_ref () . map (| s | s . item) == operation_name ; if move_op { operation = Some (op) ; } } ; } let op = match operation { Some (op) => op , None => return Err (GraphQLError :: UnknownOperationName) , } ; Ok (op) }
};
}
