// Generated macro for impl_416 (impl)
macro_rules! Depcrate_traitsimpl_416 {
() => {
// Module: crate::traits
// Provides: {"impl_416"}
// Dependencies: {}
impl < 'a > FindToken < u8 > for & 'a [u8] { fn find_token (& self , token : u8) -> bool { memchr :: memchr (token , self) . is_some () } }
};
}
