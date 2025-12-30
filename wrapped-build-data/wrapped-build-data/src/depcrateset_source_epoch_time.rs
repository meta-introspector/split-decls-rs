// Generated macro for set_SOURCE_EPOCH_TIME (function)
macro_rules! Depcrateset_SOURCE_EPOCH_TIME {
() => {
// Module: crate
// Provides: {"set_SOURCE_EPOCH_TIME"}
// Dependencies: {}
# [doc = " Sets the `SOURCE_EPOCH_TIME` env variable."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Example value: `\"1618370707\"`"] # [doc = ""] # [doc = " Reads the"] # [doc = " [`SOURCE_DATE_EPOCH`](https://reproducible-builds.org/docs/source-date-epoch/)"] # [doc = " env var if set.  Otherwise, runs `git` to get the value."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when:"] # [doc = " - `SOURCE_DATE_EPOCH` env var is non-empty and invalid"] # [doc = " - it fails to get the timestamp from `git`"] pub fn set_SOURCE_EPOCH_TIME () -> Result < () , String > { let value = get_source_time () ? ; println ! ("cargo:rustc-env=SOURCE_EPOCH_TIME={value}") ; Ok (()) }
};
}
