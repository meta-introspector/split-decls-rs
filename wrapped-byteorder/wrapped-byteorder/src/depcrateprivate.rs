// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
mod private { # [doc = " Sealed stops crates other than byteorder from implementing any traits"] # [doc = " that use it."] pub trait Sealed { } impl Sealed for super :: LittleEndian { } impl Sealed for super :: BigEndian { } }
};
}
