// Generated macro for Length (trait)
macro_rules! DepcrateLength {
() => {
// Module: crate
// Provides: {"Length"}
// Dependencies: {}
# [doc = " Obtain the length of a collection."] pub trait Length { # [doc = " Get the length of this collection."] fn len (& self) -> usize ; # [doc = " Is the collection empty?"] fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
