// Generated macro for call_unreachable (function)
macro_rules! Depcrate_derivingcall_unreachable {
() => {
// Module: crate::deriving
// Provides: {"call_unreachable"}
// Dependencies: {}
# [doc = " Constructs an expression that calls the `unreachable` intrinsic."] fn call_unreachable (cx : & ExtCtxt < '_ > , span : Span) -> Box < ast :: Expr > { let span = cx . with_def_site_ctxt (span) ; let path = cx . std_path (& [sym :: intrinsics , sym :: unreachable]) ; let call = cx . expr_call_global (span , path , ThinVec :: new ()) ; cx . expr_block (Box :: new (ast :: Block { stmts : thin_vec ! [cx . stmt_expr (call)] , id : ast :: DUMMY_NODE_ID , rules : ast :: BlockCheckMode :: Unsafe (ast :: CompilerGenerated) , span , tokens : None , })) }
};
}
