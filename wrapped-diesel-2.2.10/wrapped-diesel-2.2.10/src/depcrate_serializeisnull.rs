// Generated macro for IsNull (enum)
macro_rules! Depcrate_serializeIsNull {
() => {
// Module: crate::serialize
// Provides: {"IsNull"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq)] # [doc = " Tiny enum to make the return type of `ToSql` more descriptive"] pub enum IsNull { # [doc = " No data was written, as this type is null"] Yes , # [doc = " The value is not null"] # [doc = ""] # [doc = " This does not necessarily mean that any data was written to the buffer."] # [doc = " For example, an empty string has no data to be sent over the wire, but"] # [doc = " also is not null."] No , }
};
}
