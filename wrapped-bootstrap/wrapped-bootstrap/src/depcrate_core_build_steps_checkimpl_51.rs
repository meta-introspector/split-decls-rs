// Generated macro for impl_51 (impl)
macro_rules! Depcrate_core_build_steps_checkimpl_51 {
() => {
// Module: crate::core::build_steps::check
// Provides: {"impl_51"}
// Dependencies: {}
impl Step for CraneliftCodegenBackend { type Output = () ; const IS_HOST : bool = true ; const DEFAULT : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rustc_codegen_cranelift") . alias ("cg_clif") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CraneliftCodegenBackend { build_compiler : prepare_compiler_for_check (run . builder , run . target , Mode :: Codegen) , target : run . target , }) ; } fn run (self , builder : & Builder < '_ >) { let build_compiler = self . build_compiler . build_compiler () ; let target = self . target ; let mut cargo = builder :: Cargo :: new (builder , build_compiler , Mode :: Codegen , SourceType :: InTree , target , builder . kind ,) ; cargo . arg ("--manifest-path") . arg (builder . src . join ("compiler/rustc_codegen_cranelift/Cargo.toml")) ; rustc_cargo_env (builder , & mut cargo , target) ; self . build_compiler . configure_cargo (& mut cargo) ; let _guard = builder . msg (Kind :: Check , "rustc_codegen_cranelift" , Mode :: Codegen , build_compiler , target ,) ; let stamp = build_stamp :: codegen_backend_stamp (builder , build_compiler , target , & CodegenBackendKind :: Cranelift ,) . with_prefix ("check") ; run_cargo (builder , cargo , builder . config . free_args . clone () , & stamp , vec ! [] , true , false) ; } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: check ("rustc_codegen_cranelift" , self . target) . built_by (self . build_compiler . build_compiler ()) ,) } }
};
}
