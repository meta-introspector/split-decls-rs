// Generated macro for probe_from_env (function)
macro_rules! Depcrateprobe_from_env {
() => {
// Module: crate
// Provides: {"probe_from_env"}
// Dependencies: {}
fn probe_from_env () -> ProbeResult { let var = | name | env :: var_os (name) . map (PathBuf :: from) . filter (| p | p . exists ()) ; ProbeResult { cert_file : var (ENV_CERT_FILE) , cert_dir : var (ENV_CERT_DIR) , } }
};
}
