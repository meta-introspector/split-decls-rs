macro_rules! target_config {
    () => {
        # [doc = " Used to generate cfg variables and apply features."] # [doc = " Must express features in the way Rust understands them."] # [doc = ""] # [doc = " We do not have to worry about RUSTC_SPECIFIC_FEATURES here, those are handled outside codegen."] pub (crate) fn target_config (sess : & Session) -> TargetConfig { let target_machine = create_informational_target_machine (sess , true) ; let (unstable_target_features , target_features) = cfg_target_feature (sess , | feature | { if let Some (feat) = to_llvm_features (sess , feature) { for llvm_feature in feat { let cstr = SmallCStr :: new (llvm_feature) ; if ! unsafe { llvm :: LLVMRustHasFeature (target_machine . raw () , cstr . as_ptr ()) } { return false ; } } true } else { false } }) ; let mut cfg = TargetConfig { target_features , unstable_target_features , has_reliable_f16 : true , has_reliable_f16_math : true , has_reliable_f128 : true , has_reliable_f128_math : true , } ; update_target_reliable_float_cfg (sess , & mut cfg) ; cfg }
    };
}

target_config!()