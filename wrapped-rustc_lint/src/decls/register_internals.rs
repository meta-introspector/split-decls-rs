macro_rules! deps {
    () => {
        LintStore!();
    };
}

macro_rules! register_internals {
    () => {
        deps!();
        fn register_internals (store : & mut LintStore) { store . register_lints (& LintPassImpl :: lint_vec ()) ; store . register_early_pass (| | Box :: new (LintPassImpl)) ; store . register_lints (& DefaultHashTypes :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (DefaultHashTypes)) ; store . register_lints (& QueryStability :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (QueryStability)) ; store . register_lints (& TyTyKind :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (TyTyKind)) ; store . register_lints (& TypeIr :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (TypeIr)) ; store . register_lints (& Diagnostics :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (Diagnostics)) ; store . register_lints (& BadOptAccess :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (BadOptAccess)) ; store . register_lints (& PassByValue :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (PassByValue)) ; store . register_lints (& SpanUseEqCtxt :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (SpanUseEqCtxt)) ; store . register_lints (& SymbolInternStringLiteral :: lint_vec ()) ; store . register_late_mod_pass (| _ | Box :: new (SymbolInternStringLiteral)) ; store . register_group (false , "rustc::internal" , None , vec ! [LintId :: of (DEFAULT_HASH_TYPES) , LintId :: of (POTENTIAL_QUERY_INSTABILITY) , LintId :: of (UNTRACKED_QUERY_INFORMATION) , LintId :: of (USAGE_OF_TY_TYKIND) , LintId :: of (PASS_BY_VALUE) , LintId :: of (LINT_PASS_IMPL_WITHOUT_MACRO) , LintId :: of (USAGE_OF_QUALIFIED_TY) , LintId :: of (NON_GLOB_IMPORT_OF_TYPE_IR_INHERENT) , LintId :: of (USAGE_OF_TYPE_IR_INHERENT) , LintId :: of (USAGE_OF_TYPE_IR_TRAITS) , LintId :: of (BAD_OPT_ACCESS) , LintId :: of (SPAN_USE_EQ_CTXT) , LintId :: of (DIRECT_USE_OF_RUSTC_TYPE_IR) ,] ,) ; }
    };
}

register_internals!()