// Generated macro for impl_140 (impl)
macro_rules! Depcrate_core_build_steps_compileimpl_140 {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"impl_140"}
// Dependencies: {}
impl Std { pub fn new (build_compiler : Compiler , target : TargetSelection) -> Self { Self { target , build_compiler , crates : Default :: default () , force_recompile : false , extra_rust_args : & [] , is_for_mir_opt_tests : false , } } pub fn force_recompile (mut self , force_recompile : bool) -> Self { self . force_recompile = force_recompile ; self } # [expect (clippy :: wrong_self_convention)] pub fn is_for_mir_opt_tests (mut self , is_for_mir_opt_tests : bool) -> Self { self . is_for_mir_opt_tests = is_for_mir_opt_tests ; self } pub fn extra_rust_args (mut self , extra_rust_args : & 'static [& 'static str]) -> Self { self . extra_rust_args = extra_rust_args ; self } fn copy_extra_objects (& self , builder : & Builder < '_ > , compiler : & Compiler , target : TargetSelection ,) -> Vec < (PathBuf , DependencyType) > { let mut deps = Vec :: new () ; if ! self . is_for_mir_opt_tests { deps . extend (copy_third_party_objects (builder , compiler , target)) ; deps . extend (copy_self_contained_objects (builder , compiler , target)) ; } deps } # [doc = " Returns true if the standard library should be uplifted from stage 1."] # [doc = ""] # [doc = " Uplifting is enabled if we're building a stage2+ libstd and full bootstrap is"] # [doc = " disabled."] pub fn should_be_uplifted_from_stage_1 (builder : & Builder < '_ > , stage : u32) -> bool { stage > 1 && ! builder . config . full_bootstrap } }
};
}
