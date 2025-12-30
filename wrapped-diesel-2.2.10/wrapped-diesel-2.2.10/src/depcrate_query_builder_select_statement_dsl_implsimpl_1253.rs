// Generated macro for impl_1253 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1253 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1253"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC > CombineDsl for SelectStatement < F , S , D , W , O , LOf , G , H , LC > where Self : Query , { type Query = Self ; fn union < Rhs > (self , rhs : Rhs) -> crate :: dsl :: Union < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Union , Distinct , self , rhs . as_query ()) } fn union_all < Rhs > (self , rhs : Rhs) -> crate :: dsl :: UnionAll < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Union , All , self , rhs . as_query ()) } fn intersect < Rhs > (self , rhs : Rhs) -> crate :: dsl :: Intersect < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Intersect , Distinct , self , rhs . as_query ()) } fn intersect_all < Rhs > (self , rhs : Rhs) -> crate :: dsl :: IntersectAll < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Intersect , All , self , rhs . as_query ()) } fn except < Rhs > (self , rhs : Rhs) -> crate :: dsl :: Except < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Except , Distinct , self , rhs . as_query ()) } fn except_all < Rhs > (self , rhs : Rhs) -> crate :: dsl :: ExceptAll < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Except , All , self , rhs . as_query ()) } }
};
}
