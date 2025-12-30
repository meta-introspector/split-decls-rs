// Generated macro for impl_871 (impl)
macro_rules! Depcrate_mir_coverageinfoimpl_871 {
() => {
// Module: crate::mir::coverageinfo
// Provides: {"impl_871"}
// Dependencies: {}
impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub (crate) fn codegen_coverage (& self , bx : & mut Bx , kind : & CoverageKind , scope : SourceScope) { let instance = if let Some (inlined) = scope . inlined_instance (& self . mir . source_scopes) { self . monomorphize (inlined) } else { self . instance } ; bx . add_coverage (instance , kind) ; } }
};
}
