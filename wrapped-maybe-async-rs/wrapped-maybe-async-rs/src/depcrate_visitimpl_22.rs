// Generated macro for impl_22 (impl)
macro_rules! Depcrate_visitimpl_22 {
() => {
// Module: crate::visit
// Provides: {"impl_22"}
// Dependencies: {}
impl VisitMut for AsyncAwaitRemoval { fn visit_expr_mut (& mut self , node : & mut Expr) { visit_mut :: visit_expr_mut (self , node) ; match node { Expr :: Await (expr) => * node = (* expr . base) . clone () , Expr :: Async (expr) => { let inner = & expr . block ; let sync_expr = if let [Stmt :: Expr (expr , None)] = inner . stmts . as_slice () { expr . clone () } else { Expr :: Block (ExprBlock { attrs : expr . attrs . clone () , block : inner . clone () , label : None , }) } ; * node = sync_expr ; } _ => { } } } fn visit_item_mut (& mut self , i : & mut Item) { if let Item :: Fn (item_fn) = i { let mut inputs : Vec < (String , PathSegment) > = vec ! [] ; for param in & item_fn . sig . generics . params { if let GenericParam :: Type (type_param) = param { let generic_type_name = type_param . ident . to_string () ; for bound in & type_param . bounds { inputs . extend (search_trait_bound (& generic_type_name , bound)) ; } } } if let Some (where_clause) = & item_fn . sig . generics . where_clause { for predicate in & where_clause . predicates { if let WherePredicate :: Type (predicate_type) = predicate { let generic_type_name = if let Type :: Path (p) = & predicate_type . bounded_ty { p . path . segments [0] . ident . to_string () } else { panic ! ("Please submit an issue") ; } ; for bound in & predicate_type . bounds { inputs . extend (search_trait_bound (& generic_type_name , bound)) ; } } } } for (generic_type_name , path_seg) in & inputs { ReplaceGenericType :: replace_generic_type (i , generic_type_name , path_seg) ; } } visit_item_mut (self , i) ; } }
};
}
