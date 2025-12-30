// Generated macro for Bounds (struct)
macro_rules! Depcrate_byte_recordBounds {
() => {
// Module: crate::byte_record
// Provides: {"Bounds"}
// Dependencies: {}
# [doc = " The bounds of fields in a single record."] # [derive (Clone , Debug , Eq , PartialEq)] struct Bounds { # [doc = " The ending index of each field."] ends : Vec < usize > , # [doc = " The number of fields in this record."] # [doc = ""] # [doc = " Technically, we could drop this field and maintain an invariant that"] # [doc = " `ends.len()` is always the number of fields, but doing that efficiently"] # [doc = " requires attention to safety. We play it safe at essentially no cost."] len : usize , }
};
}
