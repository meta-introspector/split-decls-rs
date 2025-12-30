// Generated macro for Opts (struct)
macro_rules! DepcrateOpts {
() => {
// Module: crate
// Provides: {"Opts"}
// Dependencies: {}
# [derive (Default , Debug , Clone)] # [cfg_attr (feature = "clap" , derive (clap :: Args))] pub struct Opts { # [doc = " Output a `.md` file containing HTML."] # [doc = ""] # [doc = " This can be useful when producing files to be displayed on Github,"] # [doc = " as it doesn't render HTML files, but it does render Markdown files,"] # [doc = " which can contain HTML."] # [cfg_attr (feature = "clap" , arg (long))] html_in_md : bool , }
};
}
