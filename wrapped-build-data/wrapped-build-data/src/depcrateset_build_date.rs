// Generated macro for set_BUILD_DATE (function)
macro_rules! Depcrateset_BUILD_DATE {
() => {
// Module: crate
// Provides: {"set_BUILD_DATE"}
// Dependencies: {}
# [doc = " Sets the `BUILD_DATE` env variable with the current date, in UTC."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Calling this will make your build"] # [doc = " [non-reproducible](https://reproducible-builds.org/docs/timestamps/)."] # [doc = ""] # [doc = " Example value: `\"2021-04-14Z\"`"] # [allow (clippy :: missing_panics_doc)] pub fn set_BUILD_DATE () { let value = format_date (now ()) . unwrap () ; println ! ("cargo:rustc-env=BUILD_DATE={value}") ; }
};
}
