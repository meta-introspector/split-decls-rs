// Generated macro for set_BUILD_HOSTNAME (function)
macro_rules! Depcrateset_BUILD_HOSTNAME {
() => {
// Module: crate
// Provides: {"set_BUILD_HOSTNAME"}
// Dependencies: {}
# [doc = " Sets the `BUILD_HOSTNAME` env variable, with the hostname of the machine executing the build."] # [doc = ""] # [doc = " Call this from `build.rs`."] # [doc = " Use `env!` in your `main.rs` to use the variable."] # [doc = ""] # [doc = " Calling this will make your build"] # [doc = " [non-reproducible](https://reproducible-builds.org/docs/timestamps/)."] # [doc = ""] # [doc = " Example value: `\"builder2\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if it fails to execute the `hostname` command."] pub fn set_BUILD_HOSTNAME () -> Result < () , String > { let value = get_hostname () ? ; println ! ("cargo:rustc-env=BUILD_HOSTNAME={value}") ; Ok (()) }
};
}
