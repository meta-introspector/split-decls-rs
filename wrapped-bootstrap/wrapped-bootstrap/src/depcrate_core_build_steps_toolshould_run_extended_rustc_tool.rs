// Generated macro for should_run_extended_rustc_tool (function)
macro_rules! Depcrate_core_build_steps_toolshould_run_extended_rustc_tool {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"should_run_extended_rustc_tool"}
// Dependencies: {}
fn should_run_extended_rustc_tool < 'a > (run : ShouldRun < 'a > , tool_name : & 'static str , path : & 'static str , stable : bool ,) -> ShouldRun < 'a > { let builder = run . builder ; run . path (path) . default_condition (builder . config . extended && builder . config . tools . as_ref () . map_or (stable || builder . build . unstable_features () , | tools | { tools . iter () . any (| tool | match tool . as_ref () { "clippy" => tool_name == "clippy-driver" , x => tool_name == x , }) } ,) ,) }
};
}
