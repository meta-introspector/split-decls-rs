// Generated macro for impl_53 (impl)
macro_rules! Depcrate_core_build_steps_checkimpl_53 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"impl_53"}
// Dependencies: {}
impl Step for GccCodegenBackend { type Output = () ; const IS_HOST : bool = true ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rustc_codegen_gcc") . alias ("cg_gcc") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (GccCodegenBackend { build_compiler : prepare_compiler_for_check (run . builder , run . target , Mode :: Codegen) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) { if builder . build . config . vendor { println ! ("Skipping checking of `rustc_codegen_gcc` with vendoring enabled.") ; return ; } let build_compiler = self . build_compiler . build_compiler () ; let target = self . target ; let mut cargo = builder :: Cargo :: new (builder , build_compiler , Mode :: Codegen , SourceType :: InTree , target , builder . kind ,) ; cargo . arg ("--manifest-path") . arg (builder . src . join ("compiler/rustc_codegen_gcc/Cargo.toml")) ; rustc_cargo_env (builder , & mut cargo , target) ; self . build_compiler . configure_cargo (& mut cargo) ; let _guard = builder . msg (Kind :: Check , "rustc_codegen_gcc" , Mode :: Codegen , build_compiler , target) ; let stamp = build_stamp :: codegen_backend_stamp (builder , build_compiler , target , & CodegenBackendKind :: Gcc ,) . with_prefix ("check") ; run_cargo (builder , cargo , builder . config . free_args . clone () , & stamp , vec ! [] , true , false) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: check ("rustc_codegen_gcc" , self . target) . built_by (self . build_compiler . build_compiler ()) ,) } }
};
}
