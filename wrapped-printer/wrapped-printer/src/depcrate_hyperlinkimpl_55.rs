// Generated macro for impl_55 (impl)
macro_rules! Depcrate_hyperlinkimpl_55 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_55"}
// Dependencies: {}
impl HyperlinkEnvironment { # [doc = " Create a new empty hyperlink environment."] pub fn new () -> HyperlinkEnvironment { HyperlinkEnvironment :: default () } # [doc = " Set the `{host}` variable, which fills in any hostname components of"] # [doc = " a hyperlink."] # [doc = ""] # [doc = " One can get the hostname in the current environment via the `hostname`"] # [doc = " function in the `grep-cli` crate."] pub fn host (& mut self , host : Option < String >) -> & mut HyperlinkEnvironment { self . host = host ; self } # [doc = " Set the `{wslprefix}` variable, which contains the WSL distro prefix."] # [doc = " An example value is `wsl$/Ubuntu`. The distro name can typically be"] # [doc = " discovered from the `WSL_DISTRO_NAME` environment variable."] pub fn wsl_prefix (& mut self , wsl_prefix : Option < String > ,) -> & mut HyperlinkEnvironment { self . wsl_prefix = wsl_prefix ; self } }
};
}
