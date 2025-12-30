// Generated macro for VarChar (type)
macro_rules! Depcrate_sql_typesVarChar {
() => {
// Module: crate::sql_types
// Provides: {"VarChar"}
// Dependencies: {}
# [doc = " The SQL `VARCHAR` type"] # [doc = ""] # [doc = " This type is generally interchangeable with `TEXT`, so Diesel has this as an"] # [doc = " alias rather than a separate type (Diesel does not currently support"] # [doc = " implicit coercions)."] # [doc = ""] # [doc = " One notable exception to this is with arrays on PG. `TEXT[]` cannot be"] # [doc = " coerced to `VARCHAR[]`.  It is recommended that you always use `TEXT[]` if"] # [doc = " you need a string array on PG."] pub type VarChar = Text ;
};
}
