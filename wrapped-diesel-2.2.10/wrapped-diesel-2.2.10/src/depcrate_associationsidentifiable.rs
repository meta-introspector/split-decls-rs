// Generated macro for Identifiable (trait)
macro_rules! Depcrate_associationsIdentifiable {
() => {
// Module: crate::associations
// Provides: {"Identifiable"}
// Dependencies: {}
# [doc = " This trait indicates that a struct represents a single row in a database table."] # [doc = ""] # [doc = " This must be implemented to use associations."] # [doc = " Additionally, implementing this trait allows you to pass your struct to `update`"] # [doc = " (`update(&your_struct)` is equivalent to"] # [doc = " `update(YourStruct::table().find(&your_struct.primary_key())`)."] # [doc = ""] # [doc = " This trait is usually implemented on a reference to a struct,"] # [doc = " not on the struct itself. It can be [derived](derive@Identifiable)."] # [doc = ""] pub trait Identifiable : HasTable { # [doc = " The type of this struct's identifier."] # [doc = ""] # [doc = " For single-field primary keys, this is typically `&'a i32`, or `&'a String`"] # [doc = " For composite primary keys, this is typically `(&'a i32, &'a i32)`"] # [doc = " or `(&'a String, &'a String)`, etc."] type Id : Hash + Eq ; # [doc = " Returns the identifier for this record."] # [doc = ""] # [doc = " This takes `self` by value, not reference."] # [doc = " This is because composite primary keys"] # [doc = " are typically stored as multiple fields."] # [doc = " We could not return `&(String, String)` if each string is a separate field."] # [doc = ""] # [doc = " Because of Rust's rules about specifying lifetimes,"] # [doc = " this means that `Identifiable` is usually implemented on references"] # [doc = " so that we have a lifetime to use for `Id`."] fn id (self) -> Self :: Id ; }
};
}
