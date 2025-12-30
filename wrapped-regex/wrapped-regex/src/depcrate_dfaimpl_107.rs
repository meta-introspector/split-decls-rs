// Generated macro for impl_107 (impl)
macro_rules! Depcrate_dfaimpl_107 {
() => {
// Module: crate::dfa
// Provides: {"impl_107"}
// Dependencies: {}
impl fmt :: Debug for Transitions { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut fmtd = f . debug_map () ; for si in 0 .. self . num_states () { let s = si * self . num_byte_classes ; let e = s + self . num_byte_classes ; fmtd . entry (& si . to_string () , & TransitionsRow (& self . table [s .. e])) ; } fmtd . finish () } }
};
}
