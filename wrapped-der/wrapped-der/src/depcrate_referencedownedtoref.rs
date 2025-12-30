// Generated macro for OwnedToRef (trait)
macro_rules! Depcrate_referencedOwnedToRef {
() => {
// Module: crate::referenced
// Provides: {"OwnedToRef"}
// Dependencies: {}
# [doc = " A trait for borrowing data from an owned struct"] # [doc = ""] # [doc = " This converts an object owning the data to one that will borrowing the content."] # [doc = " The newly created object lifetime will be tied to the object owning the data."] # [doc = ""] # [doc = " This is similar to [`alloc::borrow::Borrow`] or [`core::convert::AsRef`] but this returns"] # [doc = " an owned structure that references directly the backing slices instead of borrowing"] # [doc = " the whole structure."] pub trait OwnedToRef { # [doc = " The resulting type referencing back to Self"] type Borrowed < 'a > where Self : 'a ; # [doc = " Creates a new object referencing back to the self for storage"] fn owned_to_ref (& self) -> Self :: Borrowed < '_ > ; }
};
}
