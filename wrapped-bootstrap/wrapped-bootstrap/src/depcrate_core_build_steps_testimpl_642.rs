// Generated macro for impl_642 (impl)
macro_rules! Depcrate_core_build_steps_testimpl_642 {
() => {
// Module: crate::core::build_steps::test
// Provides: {"impl_642"}
// Dependencies: {}
impl Step for Coverage { type Output = () ; const DEFAULT : bool = true ; # [doc = " Compiletest will automatically skip the \"coverage-run\" tests if necessary."] const IS_HOST : bool = false ; fn should_run (mut run : ShouldRun < '_ >) -> ShouldRun < '_ > { run = run . suite_path (Self :: PATH) ; for mode in Self :: ALL_MODES { run = run . alias (mode) ; } run } fn make_run (run : RunConfig < '_ >) { let compiler = run . builder . compiler (run . builder . top_stage , run . build_triple ()) ; let target = run . target ; let mut modes = vec ! [] ; for path in & run . paths { match path { PathSet :: Set (_) => { for mode in Self :: ALL_MODES { if path . assert_single_path () . path == Path :: new (mode) { modes . push (mode) ; break ; } } } PathSet :: Suite (_) => { modes . extend (Self :: ALL_MODES) ; break ; } } } modes . retain (| mode | ! run . builder . config . skip . iter () . any (| skip | skip == Path :: new (mode))) ; for mode in modes { run . builder . ensure (Coverage { compiler , target , mode }) ; } } fn run (self , builder : & Builder < '_ >) { let Self { compiler , target , mode } = self ; builder . ensure (Compiletest { test_compiler : compiler , target , mode , suite : Self :: SUITE , path : Self :: PATH , compare_mode : None , }) ; } }
};
}
