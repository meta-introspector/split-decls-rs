// Generated macro for install (macro)
macro_rules! Depcrate_core_build_steps_installinstall {
() => {
// Module: crate::core::build_steps::install
// Provides: {"install"}
// Dependencies: {}
macro_rules ! install { (($ sel : ident , $ builder : ident , $ _config : ident) , $ ($ name : ident , $ condition_name : ident = $ path_or_alias : literal , $ default_cond : expr , IS_HOST : $ IS_HOST : expr , $ run_item : block $ (, $ c : ident) *;) +) => { $ (# [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct $ name { build_compiler : Compiler , target : TargetSelection , } impl $ name { # [allow (dead_code)] fn should_build (config : & Config) -> bool { config . extended && config . tools . as_ref () . map_or (true , | t | t . contains ($ path_or_alias)) } } impl Step for $ name { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = $ IS_HOST ; $ (const $ c : bool = true ;) * fn should_run (run : ShouldRun <'_ >) -> ShouldRun <'_ > { let $ _config = & run . builder . config ; run .$ condition_name ($ path_or_alias) . default_condition ($ default_cond) } fn make_run (run : RunConfig <'_ >) { run . builder . ensure ($ name { build_compiler : run . builder . compiler (run . builder . top_stage - 1 , run . builder . config . host_target) , target : run . target , }) ; } fn run ($ sel , $ builder : & Builder <'_ >) { $ run_item } }) + } }
};
}
