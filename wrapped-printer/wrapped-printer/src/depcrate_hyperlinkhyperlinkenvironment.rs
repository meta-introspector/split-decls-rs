// Generated macro for HyperlinkEnvironment (struct)
macro_rules! Depcrate_hyperlinkHyperlinkEnvironment {
() => {
// Module: crate::hyperlink
// Provides: {"HyperlinkEnvironment"}
// Dependencies: {}
# [doc = " A static environment for hyperlink interpolation."] # [doc = ""] # [doc = " This environment permits setting the values of variables used in hyperlink"] # [doc = " interpolation that are not expected to change for the lifetime of a program."] # [doc = " That is, these values are invariant."] # [doc = ""] # [doc = " Currently, this includes the hostname and a WSL distro prefix."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct HyperlinkEnvironment { host : Option < String > , wsl_prefix : Option < String > , }
};
}
