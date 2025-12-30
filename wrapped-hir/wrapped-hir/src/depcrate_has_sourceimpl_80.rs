// Generated macro for impl_80 (impl)
macro_rules! Depcrate_has_sourceimpl_80 {
() => {
// Module: crate::has_source
// Provides: {"impl_80"}
// Dependencies: {}
impl HasSource for Param < '_ > { type Ast = Either < ast :: SelfParam , ast :: Param > ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { match self . func { Callee :: Def (CallableDefId :: FunctionId (func)) => { let InFile { file_id , value } = Function { id : func } . source (db) ? ; let params = value . param_list () ? ; if let Some (self_param) = params . self_param () { if let Some (idx) = self . idx . checked_sub (1) { params . params () . nth (idx) . map (Either :: Right) } else { Some (Either :: Left (self_param)) } } else { params . params () . nth (self . idx) . map (Either :: Right) } . map (| value | InFile { file_id , value }) } Callee :: Closure (closure , _) => { let InternedClosure (owner , expr_id) = db . lookup_intern_closure (closure) ; let (_ , source_map) = db . body_with_source_map (owner) ; let ast @ InFile { file_id , value } = source_map . expr_syntax (expr_id) . ok () ? ; let root = db . parse_or_expand (file_id) ; match value . to_node (& root) { Either :: Left (ast :: Expr :: ClosureExpr (it)) => it . param_list () ? . params () . nth (self . idx) . map (Either :: Right) . map (| value | InFile { file_id : ast . file_id , value }) , _ => None , } } _ => None , } } }
};
}
