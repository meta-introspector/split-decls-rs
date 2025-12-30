// Generated macro for impl_422 (impl)
macro_rules! Depcrate_traitsimpl_422 {
() => {
// Module: crate::traits
// Provides: {"impl_422"}
// Dependencies: {}
impl < 'a > FindToken < char > for & 'a [char] { fn find_token (& self , token : char) -> bool { self . iter () . any (| i | * i == token) } }
};
}
