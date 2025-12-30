// Generated macro for LikeIsAllowedForType (trait)
macro_rules! Depcrate_expression_operatorsLikeIsAllowedForType {
() => {
// Module: crate::expression::operators
// Provides: {"LikeIsAllowedForType"}
// Dependencies: {}
# [diagnostic :: on_unimplemented (message = "Cannot use the `LIKE` operator with expressions of the type `{ST}` for the backend `{Self}`" , note = "Expressions of the type `diesel::sql_types::Text` and `diesel::sql_types::Nullable<Text>` are \n\
            allowed for all backends")] # [cfg_attr (feature = "postgres_backend" , diagnostic :: on_unimplemented (note = "Expressions of the type `diesel::sql_types::Binary` and `diesel::sql_types::Nullable<Binary>` are \n\
            allowed for the PostgreSQL backend"))] pub trait LikeIsAllowedForType < ST > : Backend { }
};
}
