// Generated macro for Options (struct)
macro_rules! DepcrateOptions {
() => {
// Module: crate
// Provides: {"Options"}
// Dependencies: {}
# [derive (Parser)] # [clap (about , version)] struct Options { # [doc = " Show all OS information."] # [clap (long)] all : bool , # [doc = " Show OS type."] # [clap (short = 't' , long = "type")] type_ : bool , # [doc = " Show OS version."] # [clap (short = 'v' , long)] os_version : bool , # [doc = " Show OS bitness."] # [clap (short , long)] bitness : bool , # [doc = " Show OS arch."] # [clap (short = 'A' , long = "Arch")] architecture : bool , }
};
}
