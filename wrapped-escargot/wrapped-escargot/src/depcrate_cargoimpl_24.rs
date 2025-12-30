// Generated macro for impl_24 (impl)
macro_rules! Depcrate_cargoimpl_24 {
() => {
// Module: crate::cargo
// Provides: {"impl_24"}
// Dependencies: {}
impl Cargo { # [doc = " Create a top-level command."] pub fn new () -> Self { Self { cmd : process :: Command :: new (cargo_bin ()) , } } # [doc = " Manually pass an argument that is unsupported."] # [doc = ""] # [doc = " Caution: Passing in a sub-command or `--` can throw off the API."] pub fn arg < S : AsRef < ffi :: OsStr > > (mut self , arg : S) -> Self { self . cmd . arg (arg) ; self } # [doc = " Manually pass arguments that are unsupported."] # [doc = ""] # [doc = " Caution: Passing in a sub-command or `--` can throw off the API."] pub fn args < I : IntoIterator < Item = S > , S : AsRef < ffi :: OsStr > > (mut self , args : I) -> Self { self . cmd . args (args) ; self } # [doc = " Run the `build` subcommand."] pub fn build (self) -> CargoBuild { self . build_with ("build") } # [doc = " Run a custom `build` subcommand."] pub fn build_with < S : AsRef < ffi :: OsStr > > (mut self , name : S) -> CargoBuild { self . cmd . arg (name) . arg ("--message-format=json") ; CargoBuild :: with_command (self . cmd) } # [doc = " Return the underlying [`process::Command`]"] pub fn into_command (self) -> process :: Command { self . cmd } }
};
}
