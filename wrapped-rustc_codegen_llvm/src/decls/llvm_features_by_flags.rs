macro_rules! deps {
    () => {
        FixedX18InvalidArch!();
    };
}

macro_rules! llvm_features_by_flags {
    () => {
        deps!();
        # [doc = " The target features for compiler flags other than `-Ctarget-features`."] fn llvm_features_by_flags (sess : & Session , features : & mut Vec < String >) { target_features :: retpoline_features_by_flags (sess , features) ; if sess . opts . unstable_opts . fixed_x18 { if sess . target . arch != "aarch64" { sess . dcx () . emit_fatal (errors :: FixedX18InvalidArch { arch : & sess . target . arch }) ; } else { features . push ("+reserve-x18" . into ()) ; } } }
    };
}

llvm_features_by_flags!();