// Generated macro for Id (struct)
macro_rules! Depcrate_typesId {
() => {
// Module: crate::types
// Provides: {"Id"}
// Dependencies: {}
# [doc = " An [`ObjectId`] with access to a repository."] # [derive (Clone , Copy)] pub struct Id < 'r > { # [doc = " The actual object id"] pub (crate) inner : ObjectId , # [doc = " The owning repository."] pub repo : & 'r Repository , }
};
}
