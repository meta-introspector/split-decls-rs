// Generated macro for impl_313 (impl)
macro_rules! Depcrate_contextimpl_313 {
() => {
// Module: crate::context
// Provides: {"impl_313"}
// Dependencies: {}
impl < 'gcc , 'tcx > HasX86AbiOpt for CodegenCx < 'gcc , 'tcx > { fn x86_abi_opt (& self) -> X86Abi { X86Abi { regparm : self . tcx . sess . opts . unstable_opts . regparm , reg_struct_return : self . tcx . sess . opts . unstable_opts . reg_struct_return , } } }
};
}
