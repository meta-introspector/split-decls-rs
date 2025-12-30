// Generated macro for macro_23 (macro)
macro_rules! Depcrate_compressmacro_23 {
() => {
// Module: crate::compress
// Provides: {"macro_23"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (feature = "force-soft")] { mod soft ; use soft :: compress as compress_inner ; } else if # [cfg (target_arch = "loongarch64")] { mod loongarch64_asm ; use loongarch64_asm :: compress as compress_inner ; } else { mod soft ; use soft :: compress as compress_inner ; } }
};
}
