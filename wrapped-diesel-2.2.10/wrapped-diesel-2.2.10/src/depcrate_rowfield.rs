// Generated macro for Field (trait)
macro_rules! Depcrate_rowField {
() => {
// Module: crate::row
// Provides: {"Field"}
// Dependencies: {}
# [doc = " Represents a single field in a database row."] # [doc = ""] # [doc = " This trait allows retrieving information on the name of the column and on the value of the"] # [doc = " field."] pub trait Field < 'a , DB : Backend > { # [doc = " The name of the current field"] # [doc = ""] # [doc = " Returns `None` if it's an unnamed field"] fn field_name (& self) -> Option < & str > ; # [doc = " Get the value representing the current field in the raw representation"] # [doc = " as it is transmitted by the database"] fn value (& self) -> Option < DB :: RawValue < '_ > > ; # [doc = " Checks whether this field is null or not."] fn is_null (& self) -> bool { self . value () . is_none () } }
};
}
