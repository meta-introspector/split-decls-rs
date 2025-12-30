// Generated macro for impl_814 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_814 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_814"}
// Dependencies: {}
impl < Combinator , Rule , Source , Rhs , DB : Backend > QueryFragment < DB > for CombinationClause < Combinator , Rule , Source , Rhs > where Combinator : QueryFragment < DB > , Rule : QueryFragment < DB > , ParenthesisWrapper < Source > : QueryFragment < DB > , ParenthesisWrapper < Rhs > : QueryFragment < DB > , DB : Backend + SupportsCombinationClause < Combinator , Rule > + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . source . walk_ast (out . reborrow ()) ? ; self . combinator . walk_ast (out . reborrow ()) ? ; self . duplicate_rule . walk_ast (out . reborrow ()) ? ; self . rhs . walk_ast (out) } }
};
}
