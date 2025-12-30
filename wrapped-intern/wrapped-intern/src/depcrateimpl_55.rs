// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
# [allow (clippy :: new_without_default , reason = "this a const fn, so it can't be default yet. See <https://github.com/rust-lang/rust/issues/63065>")] impl < T : ? Sized > InternStorage < T > { pub const fn new () -> Self { Self { map : OnceLock :: new () } } }
};
}
