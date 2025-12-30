// Generated macro for assert_ty_bounds (function)
macro_rules! Depcrate_derivingassert_ty_bounds {
() => {
// Module: crate::deriving
// Provides: {"assert_ty_bounds"}
// Dependencies: {}
fn assert_ty_bounds (cx : & ExtCtxt < '_ > , stmts : & mut ThinVec < ast :: Stmt > , ty : Box < ast :: Ty > , span : Span , assert_path : & [Symbol] ,) { let span = cx . with_def_site_ctxt (span) ; let assert_path = cx . path_all (span , true , cx . std_path (assert_path) , vec ! [GenericArg :: Type (ty)]) ; stmts . push (cx . stmt_let_type_only (span , cx . ty_path (assert_path))) ; }
};
}
