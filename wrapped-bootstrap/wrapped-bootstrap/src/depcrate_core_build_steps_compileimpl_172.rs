// Generated macro for impl_172 (impl)
macro_rules! Depcrate_core_build_steps_compileimpl_172 {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"impl_172"}
// Dependencies: {}
impl Step for CraneliftCodegenBackend { type Output = BuildStamp ; const IS_HOST : bool = true ; fn should_run (run : ShouldRun < '_ >) -> ShouldRun < '_ > { run . alias ("rustc_codegen_cranelift") . alias ("cg_clif") } fn make_run (run : RunConfig < '_ >) { run . builder . ensure (CraneliftCodegenBackend { compilers : RustcPrivateCompilers :: new (run . builder , run . builder . top_stage , run . target) , }) ; } fn run (self , builder : & Builder < '_ >) -> Self :: Output { let target = self . compilers . target () ; let build_compiler = self . compilers . build_compiler () ; let stamp = build_stamp :: codegen_backend_stamp (builder , build_compiler , target , & CodegenBackendKind :: Cranelift ,) ; if builder . config . keep_stage . contains (& build_compiler . stage) { trace ! ("`keep-stage` requested") ; builder . info ("WARNING: Using a potentially old codegen backend. \
                This may not behave well." ,) ; return stamp ; } let mut cargo = builder :: Cargo :: new (builder , build_compiler , Mode :: Codegen , SourceType :: InTree , target , Kind :: Build ,) ; cargo . arg ("--manifest-path") . arg (builder . src . join ("compiler/rustc_codegen_cranelift/Cargo.toml")) ; rustc_cargo_env (builder , & mut cargo , target) ; let _guard = builder . msg (Kind :: Build , "codegen backend cranelift" , Mode :: Codegen , build_compiler , target ,) ; let files = run_cargo (builder , cargo , vec ! [] , & stamp , vec ! [] , false , false) ; write_codegen_backend_stamp (stamp , files , builder . config . dry_run ()) } fn metadata (& self) -> Option < StepMetadata > { Some (StepMetadata :: build ("rustc_codegen_cranelift" , self . compilers . target ()) . built_by (self . compilers . build_compiler ()) ,) } }
};
}
