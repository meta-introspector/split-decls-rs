// Generated macro for Tag (struct)
macro_rules! Depcrate_typesTag {
() => {
// Module: crate::types
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " A decoded tag object with access to its owning repository."] # [derive (Clone)] pub struct Tag < 'repo > { # [doc = " The id of the tree"] pub id : ObjectId , # [doc = " The fully decoded tag data"] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
};
}
