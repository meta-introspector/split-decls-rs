// Generated macro for impl_1773 (impl)
macro_rules! Depcrate_query_source_aliasing_aliased_fieldimpl_1773 {
() => {
// Module: crate::query_source::aliasing::aliased_field
// Provides: {"impl_1773"}
// Dependencies: {}
impl < S , C , DB > QueryFragment < DB > for AliasedField < S , C > where S : AliasSource , DB : Backend , C : Column < Table = S :: Target > , { fn walk_ast < 'b > (& 'b self , mut pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { pass . push_identifier (S :: NAME) ? ; pass . push_sql (".") ; pass . push_identifier (C :: NAME) ? ; Ok (()) } }
};
}
