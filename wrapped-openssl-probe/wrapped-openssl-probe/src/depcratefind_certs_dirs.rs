// Generated macro for find_certs_dirs (function)
macro_rules! Depcratefind_certs_dirs {
() => {
// Module: crate
// Provides: {"find_certs_dirs"}
// Dependencies: {}
# [doc = " Probe the system for the directory in which CA certificates should likely be"] # [doc = " found."] # [doc = ""] # [doc = " This will only search known system locations."] # [doc (hidden)] # [deprecated (note = "use `candidate_cert_dirs` instead")] pub fn find_certs_dirs () -> Vec < PathBuf > { candidate_cert_dirs () . map (Path :: to_path_buf) . collect () }
};
}
