// Generated macro for copy_link_tool_bin (function)
macro_rules! Depcrate_core_build_steps_toolcopy_link_tool_bin {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"copy_link_tool_bin"}
// Dependencies: {}
# [doc = " Links a built tool binary with the given `name` from the build directory to the"] # [doc = " tools directory."] fn copy_link_tool_bin (builder : & Builder < '_ > , build_compiler : Compiler , target : TargetSelection , mode : Mode , name : & str ,) -> PathBuf { let cargo_out = builder . cargo_out (build_compiler , mode , target) . join (exe (name , target)) ; let bin = builder . tools_dir (build_compiler) . join (exe (name , target)) ; builder . copy_link (& cargo_out , & bin , FileType :: Executable) ; bin }
};
}
