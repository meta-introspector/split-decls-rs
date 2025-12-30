// Generated macro for install (function)
macro_rules! Depcrateinstall {
() => {
// Module: crate
// Provides: {"install"}
// Dependencies: {}
# [doc = " Install the default panic and error report hooks"] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " This function must be called to enable the customization of `eyre::Report`"] # [doc = " provided by `color-eyre`. This function should be called early, ideally"] # [doc = " before any errors could be encountered."] # [doc = ""] # [doc = " Only the first install will succeed. Calling this function after another"] # [doc = " report handler has been installed will cause an error. **Note**: This"] # [doc = " function _must_ be called before any `eyre::Report`s are constructed to"] # [doc = " prevent the default handler from being installed."] # [doc = ""] # [doc = " Installing a global theme in `color_spantrace` manually (by calling"] # [doc = " `color_spantrace::set_theme` or `color_spantrace::colorize` before"] # [doc = " `install` is called) will result in an error if this function is called."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use color_eyre::eyre::Result;"] # [doc = ""] # [doc = " fn main() -> Result<()> {"] # [doc = "     color_eyre::install()?;"] # [doc = ""] # [doc = "     // ..."] # [doc = "     # Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn install () -> Result < () , crate :: eyre :: Report > { config :: HookBuilder :: default () . install () }
};
}
