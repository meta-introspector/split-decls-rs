// Generated macro for Adjust (enum)
macro_rules! Depcrate_inferAdjust {
() => {
// Module: crate::infer
// Provides: {"Adjust"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq , Hash)] pub enum Adjust { # [doc = " Go from ! to any type."] NeverToAny , # [doc = " Dereference once, producing a place."] Deref (Option < OverloadedDeref >) , # [doc = " Take the address and produce either a `&` or `*` pointer."] Borrow (AutoBorrow) , Pointer (PointerCast) , }
};
}
