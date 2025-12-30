// Generated macro for Options (struct)
macro_rules! Depcrate_file_includes_typesOptions {
() => {
// Module: crate::file::includes::types
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options to handle includes, like `include.path` or `includeIf.<condition>.path`,"] # [derive (Clone , Copy)] pub struct Options < 'a > { # [doc = " The maximum allowed length of the file include chain built by following nested resolve_includes where base level is depth = 0."] pub max_depth : u8 , # [doc = " When max depth is exceeded while following nested includes,"] # [doc = " return an error if true or silently stop following resolve_includes."] # [doc = ""] # [doc = " Setting this value to false allows to read configuration with cycles,"] # [doc = " which otherwise always results in an error."] pub err_on_max_depth_exceeded : bool , # [doc = " If true, default false, failing to interpolate paths will result in an error."] # [doc = ""] # [doc = " Interpolation also happens if paths in conditional includes can't be interpolated."] pub err_on_interpolation_failure : bool , # [doc = " If true, default true, configuration not originating from a path will cause errors when trying to resolve"] # [doc = " relative include paths (which would require the including configuration's path)."] pub err_on_missing_config_path : bool , # [doc = " Used during path interpolation, both for include paths before trying to read the file, and for"] # [doc = " paths used in conditional `gitdir` includes."] pub interpolate : interpolate :: Context < 'a > , # [doc = " Additional context for conditional includes to work."] pub conditional : conditional :: Context < 'a > , }
};
}
