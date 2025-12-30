// Generated macro for impl_763 (impl)
macro_rules! Depcrate_core_build_steps_toolimpl_763 {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"impl_763"}
// Dependencies: {}
impl Builder < '_ > { # [doc = " Gets a `BootstrapCommand` which is ready to run `tool` in `stage` built for"] # [doc = " `host`."] pub fn tool_cmd (& self , tool : Tool) -> BootstrapCommand { let mut cmd = command (self . tool_exe (tool)) ; let compiler = self . compiler (0 , self . config . host_target) ; let host = & compiler . host ; let mut lib_paths : Vec < PathBuf > = vec ! [self . cargo_out (compiler , Mode :: ToolBootstrap , * host) . join ("deps")] ; if compiler . host . is_msvc () { let curpaths = env :: var_os ("PATH") . unwrap_or_default () ; let curpaths = env :: split_paths (& curpaths) . collect :: < Vec < _ > > () ; for (k , v) in self . cc [& compiler . host] . env () { if k != "PATH" { continue ; } for path in env :: split_paths (v) { if ! curpaths . contains (& path) { lib_paths . push (path) ; } } } } add_dylib_path (lib_paths , & mut cmd) ; cmd . env ("RUSTC" , & self . initial_rustc) ; cmd } }
};
}
