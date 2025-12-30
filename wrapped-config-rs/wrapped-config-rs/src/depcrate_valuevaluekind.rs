// Generated macro for ValueKind (enum)
macro_rules! Depcrate_valueValueKind {
() => {
// Module: crate::value
// Provides: {"ValueKind"}
// Dependencies: {}
# [doc = " Underlying kind of the configuration value."] # [doc = ""] # [doc = " Standard operations on a [`Value`] by users of this crate do not require"] # [doc = " knowledge of [`ValueKind`]. Introspection of underlying kind is only required"] # [doc = " when the configuration values are unstructured or do not have known types."] # [derive (Debug , Clone , PartialEq , Default)] pub enum ValueKind { # [default] Nil , Boolean (bool) , I64 (i64) , I128 (i128) , U64 (u64) , U128 (u128) , Float (f64) , String (String) , Table (Table) , Array (Array) , }
};
}
