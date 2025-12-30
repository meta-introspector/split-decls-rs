// Generated macro for impl_433 (impl)
macro_rules! Depcrate_traitsimpl_433 {
() => {
// Module: crate::traits
// Provides: {"impl_433"}
// Dependencies: {}
impl < const N : usize > FindToken < u8 > for [u8 ; N] { fn find_token (& self , token : u8) -> bool { memchr :: memchr (token , & self [..]) . is_some () } }
};
}
