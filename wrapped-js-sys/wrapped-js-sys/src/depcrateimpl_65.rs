// Generated macro for impl_65 (impl)
macro_rules! Depcrateimpl_65 {
() => {
// Module: crate
// Provides: {"impl_65"}
// Dependencies: {}
impl FromStr for BigInt { type Err = Error ; # [inline] fn from_str (s : & str) -> Result < Self , Self :: Err > { BigInt :: new (& s . into ()) } }
};
}
