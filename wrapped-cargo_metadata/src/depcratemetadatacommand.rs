// Generated macro for MetadataCommand (struct)
macro_rules! DepcrateMetadataCommand {
() => {
// Module: crate
// Provides: {"MetadataCommand"}
// Dependencies: {}
# [doc = " A builder for configuring `cargo metadata` invocation."] # [derive (Debug , Clone , Default)] pub struct MetadataCommand { # [doc = " Path to `cargo` executable.  If not set, this will use the"] # [doc = " the `$CARGO` environment variable, and if that is not set, will"] # [doc = " simply be `cargo`."] cargo_path : Option < PathBuf > , # [doc = " Path to `Cargo.toml`"] manifest_path : Option < PathBuf > , # [doc = " Current directory of the `cargo metadata` process."] current_dir : Option < PathBuf > , # [doc = " Output information only about workspace members and don't fetch dependencies."] no_deps : bool , # [doc = " Collections of `CargoOpt::SomeFeatures(..)`"] features : Vec < String > , # [doc = " Latched `CargoOpt::AllFeatures`"] all_features : bool , # [doc = " Latched `CargoOpt::NoDefaultFeatures`"] no_default_features : bool , # [doc = " Arbitrary command line flags to pass to `cargo`. These will be added"] # [doc = " to the end of the command line invocation."] other_options : Vec < String > , # [doc = " Arbitrary environment variables to set or remove (depending on"] # [doc = " [`Option`] value) when running `cargo`. These will be merged into the"] # [doc = " calling environment, overriding any which clash."] env : BTreeMap < OsString , Option < OsString > > , # [doc = " Show stderr"] verbose : bool , }
};
}
