// Generated macro for prepare_clone (function)
macro_rules! Depcrateprepare_clone {
() => {
// Module: crate
// Provides: {"prepare_clone"}
// Dependencies: {}
# [doc = " Create a platform for configuring a clone with main working tree from `url` to the local `path`, using default options for opening it"] # [doc = " (but amended with using configuration from the git installation to ensure all authentication options are honored)."] # [doc = ""] # [doc = " See [`clone::PrepareFetch::new()`] for a function to take full control over all options."] # [allow (clippy :: result_large_err)] pub fn prepare_clone < Url , E > (url : Url , path : impl AsRef < std :: path :: Path >) -> Result < clone :: PrepareFetch , clone :: Error > where Url : std :: convert :: TryInto < gix_url :: Url , Error = E > , gix_url :: parse :: Error : From < E > , { clone :: PrepareFetch :: new (url , path , create :: Kind :: WithWorktree , create :: Options :: default () , open_opts_with_git_binary_config () ,) }
};
}
