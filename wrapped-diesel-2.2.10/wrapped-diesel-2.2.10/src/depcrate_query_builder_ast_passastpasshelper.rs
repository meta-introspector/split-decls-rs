// Generated macro for AstPassHelper (trait)
macro_rules! Depcrate_query_builder_ast_passAstPassHelper {
() => {
// Module: crate::query_builder::ast_pass
// Provides: {"AstPassHelper"}
// Dependencies: {}
# [doc = " This is an internal extension trait with methods required for"] # [doc = " `#[derive(MultiConnection)]`"] pub trait AstPassHelper < 'a , 'b , DB > where DB : Backend , DB :: QueryBuilder : 'a , DB :: MetadataLookup : 'a , 'b : 'a , { # [doc = " This function converts the given `AstPass` instance to"] # [doc = " an `AstPass` instance for another database system. This requires that the"] # [doc = " given instance contains compatible BindCollector/QueryBuilder/… implementations"] # [doc = " for the target backend. We use explicit conversion functions here instead of relaying on"] # [doc = " `From` impls because generating them as part of `#[derive(MultiConnection)]` is not possible"] # [doc = " due to [compiler bugs](https://github.com/rust-lang/rust/issues/100712)"] fn cast_database < DB2 > (self , convert_bind_collector : impl Fn (& 'a mut DB :: BindCollector < 'b >) -> & 'a mut DB2 :: BindCollector < 'b > , convert_query_builder : impl Fn (& mut DB :: QueryBuilder) -> & mut DB2 :: QueryBuilder , convert_backend : impl Fn (& DB) -> & DB2 , convert_lookup : impl Fn (& 'a mut DB :: MetadataLookup) -> & 'a mut DB2 :: MetadataLookup ,) -> AstPass < 'a , 'b , DB2 > where DB2 : Backend , DB2 :: QueryBuilder : 'a , DB2 :: MetadataLookup : 'a , 'b : 'a ; # [doc = " This function allows to access the inner bind collector if"] # [doc = " this `AstPass` represents a collect binds pass."] fn bind_collector (& mut self) -> Option < (& mut DB :: BindCollector < 'b > , & mut DB :: MetadataLookup) > ; }
};
}
