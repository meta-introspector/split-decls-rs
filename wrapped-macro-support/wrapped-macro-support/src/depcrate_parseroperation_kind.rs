// Generated macro for operation_kind (function)
macro_rules! Depcrate_parseroperation_kind {
() => {
// Module: crate::parser
// Provides: {"operation_kind"}
// Dependencies: {}
fn operation_kind (opts : & BindgenAttrs) -> ast :: OperationKind { let mut operation_kind = ast :: OperationKind :: Regular ; if opts . this () . is_some () { operation_kind = ast :: OperationKind :: RegularThis ; } if let Some (g) = opts . getter () { operation_kind = ast :: OperationKind :: Getter (g . clone ()) ; } if let Some (s) = opts . setter () { operation_kind = ast :: OperationKind :: Setter (s . clone ()) ; } if opts . indexing_getter () . is_some () { operation_kind = ast :: OperationKind :: IndexingGetter ; } if opts . indexing_setter () . is_some () { operation_kind = ast :: OperationKind :: IndexingSetter ; } if opts . indexing_deleter () . is_some () { operation_kind = ast :: OperationKind :: IndexingDeleter ; } operation_kind }
};
}
