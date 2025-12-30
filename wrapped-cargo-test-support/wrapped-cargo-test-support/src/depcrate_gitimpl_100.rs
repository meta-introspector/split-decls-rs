// Generated macro for impl_100 (impl)
macro_rules! Depcrate_gitimpl_100 {
() => {
// Module: crate::git
// Provides: {"impl_100"}
// Dependencies: {}
impl Repository { pub fn root (& self) -> & Path { self . 0 . workdir () . unwrap () } pub fn url (& self) -> Url { self . 0 . workdir () . unwrap () . to_url () } pub fn revparse_head (& self) -> String { self . 0 . revparse_single ("HEAD") . expect ("revparse HEAD") . id () . to_string () } }
};
}
