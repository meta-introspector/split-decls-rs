// Generated macro for from_ast_method_kind (function)
macro_rules! Depcrate_encodefrom_ast_method_kind {
() => {
// Module: crate::encode
// Provides: {"from_ast_method_kind"}
// Dependencies: {}
fn from_ast_method_kind < 'a > (function : & 'a ast :: Function , intern : & 'a Interner , method_kind : & 'a ast :: MethodKind ,) -> Result < MethodKind < 'a > , Diagnostic > { Ok (match method_kind { ast :: MethodKind :: Constructor => MethodKind :: Constructor , ast :: MethodKind :: Operation (ast :: Operation { is_static , kind }) => { let is_static = * is_static ; let kind = match kind { ast :: OperationKind :: Getter (g) => { let g = g . as_ref () . map (| g | intern . intern_str (g)) ; OperationKind :: Getter (g . unwrap_or_else (| | function . infer_getter_property ())) } ast :: OperationKind :: Regular => OperationKind :: Regular , ast :: OperationKind :: RegularThis => OperationKind :: RegularThis , ast :: OperationKind :: Setter (s) => { let s = s . as_ref () . map (| s | intern . intern_str (s)) ; OperationKind :: Setter (match s { Some (s) => s , None => intern . intern_str (& function . infer_setter_property () ?) , }) } ast :: OperationKind :: IndexingGetter => OperationKind :: IndexingGetter , ast :: OperationKind :: IndexingSetter => OperationKind :: IndexingSetter , ast :: OperationKind :: IndexingDeleter => OperationKind :: IndexingDeleter , } ; MethodKind :: Operation (Operation { is_static , kind }) } }) }
};
}
