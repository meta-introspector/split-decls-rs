// Generated macro for expand (function)
macro_rules! Depcrate_alloc_error_handlerexpand {
() => {
// Module: crate::alloc_error_handler
// Provides: {"expand"}
// Dependencies: {}
pub (crate) fn expand (ecx : & mut ExtCtxt < '_ > , _span : Span , meta_item : & ast :: MetaItem , item : Annotatable ,) -> Vec < Annotatable > { check_builtin_macro_attribute (ecx , meta_item , sym :: alloc_error_handler) ; let orig_item = item . clone () ; let (item , ident , is_stmt , sig_span) = if let Annotatable :: Item (item) = & item && let ItemKind :: Fn (fn_kind) = & item . kind { (item , fn_kind . ident , false , ecx . with_def_site_ctxt (fn_kind . sig . span)) } else if let Annotatable :: Stmt (stmt) = & item && let StmtKind :: Item (item) = & stmt . kind && let ItemKind :: Fn (fn_kind) = & item . kind { (item , fn_kind . ident , true , ecx . with_def_site_ctxt (fn_kind . sig . span)) } else { ecx . dcx () . emit_err (errors :: AllocErrorMustBeFn { span : item . span () }) ; return vec ! [orig_item] ; } ; let span = ecx . with_def_site_ctxt (item . span) ; let stmts = thin_vec ! [generate_handler (ecx , ident , span , sig_span)] ; let const_ty = ecx . ty (sig_span , TyKind :: Tup (ThinVec :: new ())) ; let const_body = ecx . expr_block (ecx . block (span , stmts)) ; let const_item = ecx . item_const (span , Ident :: new (kw :: Underscore , span) , const_ty , const_body) ; let const_item = if is_stmt { Annotatable :: Stmt (Box :: new (ecx . stmt_item (span , const_item))) } else { Annotatable :: Item (const_item) } ; vec ! [orig_item , const_item] }
};
}
