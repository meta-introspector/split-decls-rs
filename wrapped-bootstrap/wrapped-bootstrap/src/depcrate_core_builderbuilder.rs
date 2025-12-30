// Generated macro for Builder (struct)
macro_rules! Depcrate_core_builderBuilder {
() => {
// Module: crate::core::builder
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " Builds and performs different [`Self::kind`]s of stuff and actions, taking"] # [doc = " into account build configuration from e.g. bootstrap.toml."] pub struct Builder < 'a > { # [doc = " Build configuration from e.g. bootstrap.toml."] pub build : & 'a Build , # [doc = " The stage to use. Either implicitly determined based on subcommand, or"] # [doc = " explicitly specified with `--stage N`. Normally this is the stage we"] # [doc = " use, but sometimes we want to run steps with a lower stage than this."] pub top_stage : u32 , # [doc = " What to build or what action to perform."] pub kind : Kind , # [doc = " A cache of outputs of [`Step`]s so we can avoid running steps we already"] # [doc = " ran."] cache : Cache , # [doc = " A stack of [`Step`]s to run before we can run this builder. The output"] # [doc = " of steps is cached in [`Self::cache`]."] stack : RefCell < Vec < Box < dyn AnyDebug > > > , # [doc = " The total amount of time we spent running [`Step`]s in [`Self::stack`]."] time_spent_on_dependencies : Cell < Duration > , # [doc = " The paths passed on the command line. Used by steps to figure out what"] # [doc = " to do. For example: with `./x check foo bar` we get `paths=[\"foo\","] # [doc = " \"bar\"]`."] pub paths : Vec < PathBuf > , # [doc = " Cached list of submodules from self.build.src."] submodule_paths_cache : OnceLock < Vec < String > > , }
};
}
