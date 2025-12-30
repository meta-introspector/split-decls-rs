// Generated macro for impl_9 (impl)
macro_rules! Depcrate_configimpl_9 {
() => {
// Module: crate::config
// Provides: {"impl_9"}
// Dependencies: {}
impl LintcheckConfig { pub fn new () -> Self { let mut config = LintcheckConfig :: parse () ; let filename : PathBuf = config . sources_toml_path . file_stem () . unwrap () . into () ; config . lintcheck_results_path = PathBuf :: from (format ! ("lintcheck-logs/{}_logs.{}" , filename . display () , config . format . file_extension () ,)) ; if config . max_jobs == 0 { config . max_jobs = if config . fix || config . recursive { 1 } else { std :: thread :: available_parallelism () . map_or (1 , NonZero :: get) } ; } for lint_name in & mut config . lint_filter { * lint_name = format ! ("clippy::{}" , lint_name . strip_prefix ("clippy::") . unwrap_or (lint_name) . replace ('_' , "-")) ; } config } }
};
}
