// Generated macro for TypeOwnershipStyle (enum)
macro_rules! Depcrate_interfaceTypeOwnershipStyle {
() => {
// Module: crate::interface
// Provides: {"TypeOwnershipStyle"}
// Dependencies: {}
# [doc = " The style of ownership of a type, used to initially create a `TypeMode` and"] # [doc = " stored internally within it as well."] # [derive (Debug , Copy , Clone , PartialEq)] enum TypeOwnershipStyle { # [doc = " This style means owned things are printed such as `Vec<T>` and `String`."] # [doc = ""] # [doc = " Note that this primarily applies to lists."] Owned , # [doc = " This style means that lists/strings are `&[T]` and `&str`."] # [doc = ""] # [doc = " Note that this primarily applies to lists."] Borrowed , # [doc = " This style means that the top-level of a type is borrowed but all other"] # [doc = " layers are `Owned`."] # [doc = ""] # [doc = " This is used for parameters in the \"owning\" mode of generation to"] # [doc = " imports. It's easy enough to create a `&T` at the root layer but it's"] # [doc = " more difficult to create `&T` stored within a `U`, for example."] OnlyTopBorrowed , }
};
}
