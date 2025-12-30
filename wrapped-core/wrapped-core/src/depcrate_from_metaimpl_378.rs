// Generated macro for impl_378 (impl)
macro_rules! Depcrate_from_metaimpl_378 {
() => {
// Module: crate::from_meta
// Provides: {"impl_378"}
// Dependencies: {}
impl FromMeta for bool { fn from_word () -> Result < Self > { Ok (true) } # [allow (clippy :: wrong_self_convention)] fn from_bool (value : bool) -> Result < Self > { Ok (value) } fn from_string (value : & str) -> Result < Self > { value . parse () . map_err (| _ | Error :: unknown_value (value)) } }
};
}
