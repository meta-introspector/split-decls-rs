// Generated macro for CguCoverageContext (struct)
macro_rules! Depcrate_coverageinfoCguCoverageContext {
() => {
// Module: crate::coverageinfo
// Provides: {"CguCoverageContext"}
// Dependencies: {}
# [doc = " Extra per-CGU context/state needed for coverage instrumentation."] pub (crate) struct CguCoverageContext < 'll , 'tcx > { # [doc = " Associates function instances with an LLVM global that holds the"] # [doc = " function's symbol name, as needed by LLVM coverage intrinsics."] # [doc = ""] # [doc = " Instances in this map are also considered \"used\" for the purposes of"] # [doc = " emitting covfun records. Every covfun record holds a hash of its"] # [doc = " symbol name, and `llvm-cov` will exit fatally if it can't resolve that"] # [doc = " hash back to an entry in the binary's `__llvm_prf_names` linker section."] pub (crate) pgo_func_name_var_map : RefCell < FxIndexMap < Instance < 'tcx > , & 'll llvm :: Value > > , covfun_section_name : OnceCell < CString > , }
};
}
