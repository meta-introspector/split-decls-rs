// Generated macro for Cascade (struct)
macro_rules! Depcrate_helperCascade {
() => {
// Module: crate::helper
// Provides: {"Cascade"}
// Dependencies: {}
# [doc = " A list of helper programs to run in order to obtain credentials."] # [allow (dead_code)] # [derive (Debug)] pub struct Cascade { # [doc = " The programs to run in order to obtain credentials"] pub programs : Vec < Program > , # [doc = " If true, stderr is enabled when `programs` are run, which is the default."] pub stderr : bool , # [doc = " If true, http(s) urls will take their path portion into account when obtaining credentials. Default is false."] # [doc = " Other protocols like ssh will always use the path portion."] pub use_http_path : bool , # [doc = " If true, default false, when getting credentials, we will set a bogus password to only obtain the user name."] # [doc = " Storage and cancellation work the same, but without a password set."] pub query_user_only : bool , }
};
}
