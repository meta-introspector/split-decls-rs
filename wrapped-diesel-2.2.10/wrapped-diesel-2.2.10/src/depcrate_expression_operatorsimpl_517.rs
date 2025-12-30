// Generated macro for impl_517 (impl)
macro_rules! Depcrate_expression_operatorsimpl_517 {
() => {
// Module: crate::expression::operators
// Provides: {"impl_517"}
// Dependencies: {}
impl < T , U , DB > crate :: query_builder :: QueryFragment < DB > for Like < T , U > where T : crate :: query_builder :: QueryFragment < DB > + crate :: Expression , U : crate :: query_builder :: QueryFragment < DB > , DB : crate :: backend :: Backend , DB : LikeIsAllowedForType < T :: SqlType > , { fn walk_ast < 'b > (& 'b self , mut out : crate :: query_builder :: AstPass < '_ , 'b , DB > ,) -> crate :: result :: QueryResult < () > { (self . left . walk_ast (out . reborrow ()) ?) ; (out . push_sql (" LIKE ")) ; (self . right . walk_ast (out . reborrow ()) ?) ; Ok (()) } }
};
}
