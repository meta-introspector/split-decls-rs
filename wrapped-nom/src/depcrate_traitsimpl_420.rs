// Generated macro for impl_420 (impl)
macro_rules! Depcrate_traitsimpl_420 {
() => {
// Module: crate::traits
// Provides: {"impl_420"}
// Dependencies: {}
impl < 'a > FindToken < char > for & 'a [u8] { fn find_token (& self , token : char) -> bool { self . iter () . any (| i | * i == token as u8) } }
};
}
