// Generated macro for IntoNullable (trait)
macro_rules! Depcrate_sql_typesIntoNullable {
() => {
// Module: crate::sql_types
// Provides: {"IntoNullable"}
// Dependencies: {}
# [doc = " Converts a type which may or may not be nullable into its nullable"] # [doc = " representation."] pub trait IntoNullable { # [doc = " The nullable representation of this type."] # [doc = ""] # [doc = " For all types except `Nullable`, this will be `Nullable<Self>`."] type Nullable ; }
};
}
