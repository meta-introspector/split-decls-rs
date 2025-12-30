// Generated macro for BoolOrNullableBool (trait)
macro_rules! Depcrate_sql_typesBoolOrNullableBool {
() => {
// Module: crate::sql_types
// Provides: {"BoolOrNullableBool"}
// Dependencies: {}
# [doc = " A marker trait for accepting expressions of the type `Bool` and"] # [doc = " `Nullable<Bool>` in the same place"] # [diagnostic :: on_unimplemented (message = "`{Self}` is neither `diesel::sql_types::Bool` nor `diesel::sql_types::Nullable<Bool>`" , note = "try to provide an expression that produces one of the expected sql types")] pub trait BoolOrNullableBool { }
};
}
