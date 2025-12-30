// Generated macro for AstPassInternals (enum)
macro_rules! Depcrate_query_builder_ast_passAstPassInternals {
() => {
// Module: crate::query_builder::ast_pass
// Provides: {"AstPassInternals"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [doc = " This is separate from the struct to cause the enum to be opaque, forcing"] # [doc = " usage of the methods provided rather than matching on the enum directly."] # [doc = " This essentially mimics the capabilities that would be available if"] # [doc = " `AstPass` were a trait."] enum AstPassInternals < 'a , 'b , DB > where DB : Backend , DB :: QueryBuilder : 'a , DB :: MetadataLookup : 'a , 'b : 'a , { ToSql (& 'a mut DB :: QueryBuilder , & 'a mut AstPassToSqlOptions) , CollectBinds { collector : & 'a mut DB :: BindCollector < 'b > , metadata_lookup : & 'a mut DB :: MetadataLookup , } , IsSafeToCachePrepared (& 'a mut bool) , DebugBinds (& 'a mut Vec < Box < dyn fmt :: Debug + 'b > >) , IsNoop (& 'a mut bool) , }
};
}
