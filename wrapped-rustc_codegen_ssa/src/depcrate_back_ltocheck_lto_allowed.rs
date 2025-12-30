// Generated macro for check_lto_allowed (function)
macro_rules! Depcrate_back_ltocheck_lto_allowed {
() => {
// Module: crate::back::lto
// Provides: {"check_lto_allowed"}
// Dependencies: {}
pub (super) fn check_lto_allowed < B : WriteBackendMethods > (cgcx : & CodegenContext < B >) { if cgcx . lto == Lto :: ThinLocal { return ; } let dcx = cgcx . create_dcx () ; for crate_type in cgcx . crate_types . iter () { if ! crate_type_allows_lto (* crate_type) { dcx . handle () . emit_fatal (LtoDisallowed) ; } else if * crate_type == CrateType :: Dylib { if ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (LtoDylib) ; } } else if * crate_type == CrateType :: ProcMacro && ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (LtoProcMacro) ; } } if cgcx . opts . cg . prefer_dynamic && ! cgcx . opts . unstable_opts . dylib_lto { dcx . handle () . emit_fatal (DynamicLinkingWithLTO) ; } }
};
}
