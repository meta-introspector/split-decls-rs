// Generated macro for IntoNotNullable (trait)
macro_rules! Depcrate_sql_typesIntoNotNullable {
() => {
// Module: crate::sql_types
// Provides: {"IntoNotNullable"}
// Dependencies: {}
# [doc = " Converts a type which may or may not be nullable into its not nullable"] # [doc = " representation."] pub trait IntoNotNullable { # [doc = " The not nullable representation of this type."] # [doc = ""] # [doc = " For `Nullable<T>`, this will be `T` otherwise the type itself"] type NotNullable ; }
};
}
