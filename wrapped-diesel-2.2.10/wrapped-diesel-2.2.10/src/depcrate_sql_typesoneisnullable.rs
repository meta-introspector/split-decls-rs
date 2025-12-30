// Generated macro for OneIsNullable (trait)
macro_rules! Depcrate_sql_typesOneIsNullable {
() => {
// Module: crate::sql_types
// Provides: {"OneIsNullable"}
// Dependencies: {}
# [doc = " Is one value of `IsNull` nullable?"] # [doc = ""] # [doc = " You should never implement this trait."] pub trait OneIsNullable < Other > { # [doc = " See the trait documentation"] type Out : OneIsNullable < is_nullable :: IsNullable > + OneIsNullable < is_nullable :: NotNull > ; }
};
}
