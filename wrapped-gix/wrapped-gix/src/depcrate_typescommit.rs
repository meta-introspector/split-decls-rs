// Generated macro for Commit (struct)
macro_rules! Depcrate_typesCommit {
() => {
// Module: crate::types
// Provides: {"Commit"}
// Dependencies: {}
# [doc = " A decoded commit object with access to its owning repository."] # [derive (Clone)] pub struct Commit < 'repo > { # [doc = " The id of the commit"] pub id : ObjectId , # [doc = " The fully decoded commit data"] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
};
}
