// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1022 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1022"}
// Dependencies: {}
impl < T : QuerySource , U , Op , Ret > InsertStatement < T , U , Op , Ret > { # [doc = " Create a new InsertStatement instance"] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] pub (crate) fn new (target : T , records : U , operator : Op , returning : Ret) -> Self { InsertStatement { into_clause : target . from_clause () , operator , target , records , returning , } } pub (crate) fn replace_values < F , V > (self , f : F) -> InsertStatement < T , V , Op , Ret > where F : FnOnce (U) -> V , { InsertStatement :: new (self . target , f (self . records) , self . operator , self . returning) } }
};
}
