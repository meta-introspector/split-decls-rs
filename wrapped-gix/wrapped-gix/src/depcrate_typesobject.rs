// Generated macro for Object (struct)
macro_rules! Depcrate_typesObject {
() => {
// Module: crate::types
// Provides: {"Object"}
// Dependencies: {}
# [doc = " A decoded object with a reference to its owning repository."] # [derive (Clone)] pub struct Object < 'repo > { # [doc = " The id of the object"] pub id : ObjectId , # [doc = " The kind of the object"] pub kind : gix_object :: Kind , # [doc = " The fully decoded object data"] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
};
}
