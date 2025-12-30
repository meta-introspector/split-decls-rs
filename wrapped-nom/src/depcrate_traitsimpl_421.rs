// Generated macro for impl_421 (impl)
macro_rules! Depcrate_traitsimpl_421 {
() => {
// Module: crate::traits
// Provides: {"impl_421"}
// Dependencies: {}
impl < 'a > FindToken < char > for & 'a str { fn find_token (& self , token : char) -> bool { self . chars () . any (| i | i == token) } }
};
}
