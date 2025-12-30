// Generated macro for rustc_env (function)
macro_rules! Depcrate_outputrustc_env {
() => {
// Module: crate::output
// Provides: {"rustc_env"}
// Dependencies: {}
# [doc = " The `rustc-env` instruction tells Cargo to set the given environment variable"] # [doc = " when compiling the package."] # [doc = ""] # [doc = " The value can be then retrieved by the"] # [doc = " [`env!` macro][env!] in the compiled crate. This is useful for embedding"] # [doc = " additional metadata in crate’s code, such as the hash of git HEAD or the"] # [doc = " unique identifier of a continuous integration server."] # [doc = ""] # [doc = " See also the [environment variables automatically included by Cargo][cargo-env]."] # [doc = ""] # [doc = " [cargo-env]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates"] # [track_caller] pub fn rustc_env (key : & str , value : & str) { if key . contains (['=' , '\n']) { panic ! ("cannot emit rustc-env: invalid key {key:?}") ; } if value . contains ('\n') { panic ! ("cannot emit rustc-env: invalid value {value:?}") ; } emit ("rustc-env" , format_args ! ("{key}={value}")) ; }
};
}
