// Generated macro for BindCollector (trait)
macro_rules! Depcrate_query_builder_bind_collectorBindCollector {
() => {
// Module: crate::query_builder::bind_collector
// Provides: {"BindCollector"}
// Dependencies: {}
# [doc = " A type which manages serializing bind parameters during query construction."] # [doc = ""] # [doc = " The only reason you would ever need to interact with this trait is if you"] # [doc = " are adding support for a new backend to Diesel. Plugins which are extending"] # [doc = " the query builder will use [`AstPass::push_bind_param`] instead."] # [doc = ""] # [doc = " [`AstPass::push_bind_param`]: crate::query_builder::AstPass::push_bind_param()"] pub trait BindCollector < 'a , DB : TypeMetadata > : Sized { # [doc = " The internal buffer type used by this bind collector"] type Buffer ; # [doc = " Serializes the given bind value, and collects the result."] fn push_bound_value < T , U > (& mut self , bind : & 'a U , metadata_lookup : & mut DB :: MetadataLookup ,) -> QueryResult < () > where DB : Backend + HasSqlType < T > , U : ToSql < T , DB > + ? Sized + 'a ; # [doc = " Push a null value with the given type information onto the bind collector"] # [doc = ""] # [diesel_derives :: __diesel_public_if (feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes")] fn push_null_value (& mut self , _metadata : DB :: TypeMetadata) -> QueryResult < () > { Ok (()) } }
};
}
