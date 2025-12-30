// Generated macro for impl_59 (impl)
macro_rules! Depcrate_argsimpl_59 {
() => {
// Module: crate::args
// Provides: {"impl_59"}
// Dependencies: {}
impl FromMeta for NewTypeName { fn from_word () -> darling :: Result < Self > { Ok (Self :: Rust) } fn from_string (value : & str) -> darling :: Result < Self > { Ok (Self :: New (value . to_string ())) } fn from_bool (value : bool) -> darling :: Result < Self > { if value { Ok (Self :: Rust) } else { Ok (Self :: Original) } } }
};
}
