// Generated macro for impl_406 (impl)
macro_rules! Depcrate_core_build_steps_installimpl_406 {
() => {
// Module: crate::core::build_steps::install
// Provides: {"impl_406"}
// Dependencies: {}
impl Step for Src { type Output = () ; const DEFAULT : bool = true ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { let config = & run . builder . config ; let cond = config . extended && config . tools . as_ref () . is_none_or (| t | t . contains ("src")) ; run . path ("src") . default_condition (cond) } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (Src { stage : run . builder . top_stage }) ; } fn run (self , builder : & Builder < '_ >) { let tarball = builder . ensure (dist :: Src) ; install_sh (builder , "src" , None , None , & tarball) ; } }
};
}
