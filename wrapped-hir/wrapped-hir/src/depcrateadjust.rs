// Generated macro for Adjust (enum)
macro_rules! DepcrateAdjust {
() => {
// Module: crate
// Provides: {"Adjust"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum Adjust { # [doc = " Go from ! to any type."] NeverToAny , # [doc = " Dereference once, producing a place."] Deref (Option < OverloadedDeref >) , # [doc = " Take the address and produce either a `&` or `*` pointer."] Borrow (AutoBorrow) , Pointer (PointerCast) , }
};
}
