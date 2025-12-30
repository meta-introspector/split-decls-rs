// Generated macro for profile (function)
macro_rules! Depcrate_inputprofile {
() => {
// Module: crate::input
// Provides: {"profile"}
// Dependencies: {}
# [doc = " `release` for release builds, `debug` for other builds."] # [doc = ""] # [doc = " This is determined based"] # [doc = " on if the [profile] inherits from the [`dev`] or [`release`] profile. Using this"] # [doc = " function is not recommended. Using other functions like [`opt_level`] provides"] # [doc = " a more correct view of the actual settings being used."] # [doc = ""] # [doc = " [profile]: https://doc.rust-lang.org/stable/cargo/reference/profiles.html"] # [doc = " [`dev`]: https://doc.rust-lang.org/stable/cargo/reference/profiles.html#dev"] # [doc = " [`release`]: https://doc.rust-lang.org/stable/cargo/reference/profiles.html#release"] # [track_caller] pub fn profile () -> String { to_string (var_or_panic ("PROFILE")) }
};
}
