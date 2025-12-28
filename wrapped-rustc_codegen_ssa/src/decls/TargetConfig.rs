macro_rules! TargetConfig {
    () => {
        # [doc = " Target-specific options that get set in `cfg(...)`."] # [doc = ""] # [doc = " RUSTC_SPECIFIC_FEATURES should be skipped here, those are handled outside codegen."] pub struct TargetConfig { # [doc = " Options to be set in `cfg(target_features)`."] pub target_features : Vec < Symbol > , # [doc = " Options to be set in `cfg(target_features)`, but including unstable features."] pub unstable_target_features : Vec < Symbol > , # [doc = " Option for `cfg(target_has_reliable_f16)`, true if `f16` basic arithmetic works."] pub has_reliable_f16 : bool , # [doc = " Option for `cfg(target_has_reliable_f16_math)`, true if `f16` math calls work."] pub has_reliable_f16_math : bool , # [doc = " Option for `cfg(target_has_reliable_f128)`, true if `f128` basic arithmetic works."] pub has_reliable_f128 : bool , # [doc = " Option for `cfg(target_has_reliable_f128_math)`, true if `f128` math calls work."] pub has_reliable_f128_math : bool , }
    };
}

TargetConfig!()