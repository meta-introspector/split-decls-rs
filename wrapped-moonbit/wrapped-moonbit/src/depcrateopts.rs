// Generated macro for Opts (struct)
macro_rules! DepcrateOpts {
() => {
// Module: crate
// Provides: {"Opts"}
// Dependencies: {}
# [derive (Default , Debug , Clone)] # [cfg_attr (feature = "clap" , derive (clap :: Parser))] pub struct Opts { # [doc = " Whether or not to derive Show for all types"] # [cfg_attr (feature = "clap" , arg (long , default_value_t = false))] pub derive_show : bool , # [doc = " Whether or not to derive Eq for all types"] # [cfg_attr (feature = "clap" , arg (long , default_value_t = false))] pub derive_eq : bool , # [doc = " Whether or not to declare as Error type for types \".*error\""] # [cfg_attr (feature = "clap" , arg (long , default_value_t = false))] pub derive_error : bool , # [doc = " Whether or not to generate stub files ; useful for update after WIT change"] # [cfg_attr (feature = "clap" , arg (long , default_value_t = false))] pub ignore_stub : bool , # [doc = " Whether or not to generate moon.mod.json ; useful if the project is part of a larger project"] # [cfg_attr (feature = "clap" , arg (long , default_value_t = false))] pub ignore_module_file : bool , # [doc = " The package/dir to generate the program entrance"] # [cfg_attr (feature = "clap" , arg (long , default_value = "gen"))] pub gen_dir : String , # [doc = " The project name ; or the package path prefix if the project is part of a larger project"] # [cfg_attr (feature = "clap" , arg (long , default_value = None))] pub project_name : Option < String > , # [cfg_attr (feature = "clap" , clap (flatten))] pub async_ : AsyncFilterSet , }
};
}
