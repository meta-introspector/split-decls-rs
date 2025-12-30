// Generated macro for impl_239 (impl)
macro_rules! Depcrate_idimpl_239 {
() => {
// Module: crate::id
// Provides: {"impl_239"}
// Dependencies: {}
impl < 'repo > Id < 'repo > { # [doc = " Obtain a platform for traversing ancestors of this commit."] pub fn ancestors (& self) -> crate :: revision :: walk :: Platform < 'repo > { crate :: revision :: walk :: Platform :: new (Some (self . inner) , self . repo) } }
};
}
