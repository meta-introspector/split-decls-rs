// Generated macro for RefToOwned (trait)
macro_rules! Depcrate_referencedRefToOwned {
() => {
// Module: crate::referenced
// Provides: {"RefToOwned"}
// Dependencies: {}
# [doc = " A trait for cloning a referenced structure and getting owned objects"] # [doc = ""] # [doc = " This is the pendant to [`OwnedToRef`]."] # [doc = ""] # [doc = " This converts an object borrowing data to one that will copy the data over and"] # [doc = " own the content."] pub trait RefToOwned < 'a > { # [doc = " The resulting type after obtaining ownership."] type Owned : OwnedToRef < Borrowed < 'a > = Self > where Self : 'a ; # [doc = " Creates a new object taking ownership of the data"] fn ref_to_owned (& self) -> Self :: Owned ; }
};
}
