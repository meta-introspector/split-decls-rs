// Generated macro for impl_32 (impl)
macro_rules! Depcrate_util_entityimpl_32 {
() => {
// Module: crate::util::entity
// Provides: {"impl_32"}
// Dependencies: {}
impl super :: TryFromValues for EntityTag { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . just_one () . and_then (EntityTag :: from_val) . ok_or_else (Error :: invalid) } }
};
}
