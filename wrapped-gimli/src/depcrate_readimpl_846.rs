// Generated macro for impl_846 (impl)
macro_rules! Depcrate_readimpl_846 {
() => {
// Module: crate::read
// Provides: {"impl_846"}
// Dependencies: {}
impl Register { pub (crate) fn from_u64 (x : u64) -> Result < Register > { let y = x as u16 ; if u64 :: from (y) == x { Ok (Register (y)) } else { Err (Error :: UnsupportedRegister (x)) } } }
};
}
