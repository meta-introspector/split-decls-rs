// Generated macro for run_tool_check_step (function)
macro_rules! Depcrate_core_build_steps_checkrun_tool_check_step {
() => {
// Module: crate::core::build_steps::check
// Provides: {"run_tool_check_step"}
// Dependencies: {}
# [doc = " Used by the implementation of `Step::run` in `tool_check_step!`."] fn run_tool_check_step (builder : & Builder < '_ > , compiler : CompilerForCheck , target : TargetSelection , path : & str , mode : Mode , allow_features : & str , extra_features : & [& str] ,) { let display_name = path . rsplit ('/') . next () . unwrap () ; let build_compiler = compiler . build_compiler () ; let extra_features = extra_features . iter () . map (| f | f . to_string ()) . collect :: < Vec < String > > () ; let mut cargo = prepare_tool_cargo (builder , build_compiler , mode , target , builder . kind , path , SourceType :: InTree , & extra_features ,) ; cargo . allow_features (allow_features) ; compiler . configure_cargo (& mut cargo) ; if display_name == "rust-analyzer" { cargo . arg ("--bins") ; cargo . arg ("--tests") ; cargo . arg ("--benches") ; } else { cargo . arg ("--all-targets") ; } let stamp = BuildStamp :: new (& builder . cargo_out (build_compiler , mode , target)) . with_prefix (& format ! ("{display_name}-check")) ; let _guard = builder . msg (builder . kind , display_name , mode , build_compiler , target) ; run_cargo (builder , cargo , builder . config . free_args . clone () , & stamp , vec ! [] , true , false) ; }
};
}
