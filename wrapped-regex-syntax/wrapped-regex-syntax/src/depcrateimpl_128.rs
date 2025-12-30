// Generated macro for impl_128 (impl)
macro_rules! Depcrateimpl_128 {
() => {
// Module: crate
// Provides: {"impl_128"}
// Dependencies: {}
impl PartialOrd < u8 > for ByteRange { # [inline] fn partial_cmp (& self , other : & u8) -> Option < Ordering > { Some (if self == other { Ordering :: Equal } else if * other > self . end { Ordering :: Greater } else { Ordering :: Less }) } }
};
}
