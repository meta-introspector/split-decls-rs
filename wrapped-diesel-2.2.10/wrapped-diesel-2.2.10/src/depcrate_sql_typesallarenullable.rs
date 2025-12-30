// Generated macro for AllAreNullable (trait)
macro_rules! Depcrate_sql_typesAllAreNullable {
() => {
// Module: crate::sql_types
// Provides: {"AllAreNullable"}
// Dependencies: {}
# [doc = " Are both values of `IsNull` are nullable?"] pub trait AllAreNullable < Other > { # [doc = " See the trait documentation"] type Out : AllAreNullable < is_nullable :: NotNull > + AllAreNullable < is_nullable :: IsNullable > ; }
};
}
