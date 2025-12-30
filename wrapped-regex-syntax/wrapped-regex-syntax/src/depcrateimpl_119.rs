// Generated macro for impl_119 (impl)
macro_rules! Depcrateimpl_119 {
() => {
// Module: crate
// Provides: {"impl_119"}
// Dependencies: {}
impl PartialOrd < char > for ClassRange { # [inline] fn partial_cmp (& self , other : & char) -> Option < Ordering > { Some (if self == other { Ordering :: Equal } else if * other > self . end { Ordering :: Greater } else { Ordering :: Less }) } }
};
}
