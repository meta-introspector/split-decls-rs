// Generated macro for err_unnamed_type (function)
macro_rules! Depcrate_macros_helpererr_unnamed_type {
() => {
// Module: crate::macros::helper
// Provides: {"err_unnamed_type"}
// Dependencies: {}
# [doc = " Generates a [`FieldError`] for the given Rust type expecting to have"] # [doc = " [`GraphQLType::name`]."] # [doc = ""] # [doc = " [`GraphQLType::name`]: crate::GraphQLType::name"] pub fn err_unnamed_type < S > (name : & str) -> FieldError < S > { FieldError :: from (format ! ("Expected `{name}` type to implement `GraphQLType::name`" ,)) }
};
}
