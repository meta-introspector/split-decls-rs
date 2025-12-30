// Generated macro for expand (function)
macro_rules! Depcrate_global_allocatorexpand {
() => {
// Module: crate::global_allocator
// Provides: {"expand"}
// Dependencies: {}
pub (crate) fn expand (ecx : & mut ExtCtxt < '_ > , _span : Span , meta_item : & ast :: MetaItem , item : Annotatable ,) -> Vec < Annotatable > { check_builtin_macro_attribute (ecx , meta_item , sym :: global_allocator) ; let orig_item = item . clone () ; let (item , ident , is_stmt , ty_span) = if let Annotatable :: Item (item) = & item && let ItemKind :: Static (box ast :: StaticItem { ident , ty , .. }) = & item . kind { (item , * ident , false , ecx . with_def_site_ctxt (ty . span)) } else if let Annotatable :: Stmt (stmt) = & item && let StmtKind :: Item (item) = & stmt . kind && let ItemKind :: Static (box ast :: StaticItem { ident , ty , .. }) = & item . kind { (item , * ident , true , ecx . with_def_site_ctxt (ty . span)) } else { ecx . dcx () . emit_err (errors :: AllocMustStatics { span : item . span () }) ; return vec ! [orig_item] ; } ; let span = ecx . with_def_site_ctxt (item . span) ; let f = AllocFnFactory { span , ty_span , global : ident , cx : ecx } ; let stmts = ALLOCATOR_METHODS . iter () . map (| method | f . allocator_fn (method)) . collect () ; let const_ty = ecx . ty (ty_span , TyKind :: Tup (ThinVec :: new ())) ; let const_body = ecx . expr_block (ecx . block (span , stmts)) ; let const_item = ecx . item_const (span , Ident :: new (kw :: Underscore , span) , const_ty , const_body) ; let const_item = if is_stmt { Annotatable :: Stmt (Box :: new (ecx . stmt_item (span , const_item))) } else { Annotatable :: Item (const_item) } ; vec ! [orig_item , const_item] }
};
}
