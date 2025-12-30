// Generated macro for EXTENSIVE_ITER_OVERRIDE (static)
macro_rules! Depcrate_run_cfgEXTENSIVE_ITER_OVERRIDE {
() => {
// Module: crate::run_cfg
// Provides: {"EXTENSIVE_ITER_OVERRIDE"}
// Dependencies: {}
# [doc = " The override value, if set by the above environment."] static EXTENSIVE_ITER_OVERRIDE : LazyLock < Option < u64 > > = LazyLock :: new (| | { env :: var (EXTENSIVE_ITER_ENV) . map (| v | v . parse () . expect ("failed to parse iteration count")) . ok () }) ;
};
}
