// Generated macro for impl_52 (impl)
macro_rules! Depcrate_kindimpl_52 {
() => {
// Module: crate::kind
// Provides: {"impl_52"}
// Dependencies: {}
impl TryFrom < u8 > for Kind { type Error = u8 ; fn try_from (value : u8) -> Result < Self , Self :: Error > { Ok (match value { 1 => Kind :: Sha1 , unknown => return Err (unknown) , }) } }
};
}
