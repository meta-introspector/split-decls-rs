// Generated macro for pair (function)
macro_rules! Depcratepair {
() => {
// Module: crate
// Provides: {"pair"}
// Dependencies: {}
# [doc = " Creates a parker and an associated unparker."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let (p, u) = parking::pair();"] # [doc = " ```"] pub fn pair () -> (Parker , Unparker) { let p = Parker :: new () ; let u = p . unparker () ; (p , u) }
};
}
