// Generated macro for prepare_clone_bare (function)
macro_rules! Depcrateprepare_clone_bare {
() => {
// Module: crate
// Provides: {"prepare_clone_bare"}
// Dependencies: {}
# [doc = " Create a platform for configuring a bare clone from `url` to the local `path`, using default options for opening it (but"] # [doc = " amended with using configuration from the git installation to ensure all authentication options are honored)."] # [doc = ""] # [doc = " See [`clone::PrepareFetch::new()`] for a function to take full control over all options."] # [allow (clippy :: result_large_err)] pub fn prepare_clone_bare < Url , E > (url : Url , path : impl AsRef < std :: path :: Path > ,) -> Result < clone :: PrepareFetch , clone :: Error > where Url : std :: convert :: TryInto < gix_url :: Url , Error = E > , gix_url :: parse :: Error : From < E > , { clone :: PrepareFetch :: new (url , path , create :: Kind :: Bare , create :: Options :: default () , open_opts_with_git_binary_config () ,) }
};
}
