// Generated macro for impl_17 (impl)
macro_rules! Depcrate_ucharimpl_17 {
() => {
// Module: crate::uchar
// Provides: {"impl_17"}
// Dependencies: {}
impl TryFrom < u32 > for PotentialCodePoint { type Error = () ; fn try_from (x : u32) -> Result < Self , () > { let [u0 , u1 , u2 , u3] = x . to_le_bytes () ; if u3 != 0 { return Err (()) ; } Ok (Self ([u0 , u1 , u2])) } }
};
}
