// Generated macro for impl_1565 (impl)
macro_rules! Depcrate_query_dsl_combine_dslimpl_1565 {
() => {
// Module: crate::query_dsl::combine_dsl
// Provides: {"impl_1565"}
// Dependencies: {}
impl < T : Table > CombineDsl for T { type Query = T :: Query ; fn union < Rhs > (self , rhs : Rhs) -> dsl :: Union < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Union , Distinct , self . as_query () , rhs . as_query ()) } fn union_all < Rhs > (self , rhs : Rhs) -> dsl :: UnionAll < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Union , All , self . as_query () , rhs . as_query ()) } fn intersect < Rhs > (self , rhs : Rhs) -> dsl :: Intersect < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Intersect , Distinct , self . as_query () , rhs . as_query ()) } fn intersect_all < Rhs > (self , rhs : Rhs) -> dsl :: IntersectAll < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Intersect , All , self . as_query () , rhs . as_query ()) } fn except < Rhs > (self , rhs : Rhs) -> dsl :: Except < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Except , Distinct , self . as_query () , rhs . as_query ()) } fn except_all < Rhs > (self , rhs : Rhs) -> dsl :: ExceptAll < Self , Rhs > where Rhs : AsQuery < SqlType = < Self :: Query as Query > :: SqlType > , { CombinationClause :: new (Except , All , self . as_query () , rhs . as_query ()) } }
};
}
