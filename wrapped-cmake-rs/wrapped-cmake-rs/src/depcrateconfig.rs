// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Builder style configuration for a pending CMake build."] pub struct Config { path : PathBuf , generator : Option < OsString > , generator_toolset : Option < OsString > , cflags : OsString , cxxflags : OsString , asmflags : OsString , defines : Vec < (OsString , OsString) > , deps : Vec < String > , target : Option < String > , host : Option < String > , out_dir : Option < PathBuf > , profile : Option < String > , configure_args : Vec < OsString > , build_args : Vec < OsString > , cmake_target : Option < String > , env : Vec < (OsString , OsString) > , static_crt : Option < bool > , uses_cxx11 : bool , always_configure : bool , no_build_target : bool , no_default_flags : bool , verbose_cmake : bool , verbose_make : bool , pic : Option < bool > , c_cfg : Option < cc :: Build > , cxx_cfg : Option < cc :: Build > , env_cache : HashMap < String , Option < OsString > > , }
};
}
