// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl Terminator { # [doc = " Checks whether the terminator is set to CRLF."] fn is_crlf (& self) -> bool { match * self { Terminator :: CRLF => true , Terminator :: Any (_) => false , } } fn equals (& self , other : u8) -> bool { match * self { Terminator :: CRLF => other == b'\r' || other == b'\n' , Terminator :: Any (b) => other == b , } } }
};
}
