macro_rules! deps {
    () => {
        ProbeResult!();
    };
}

macro_rules! probe_from_env {
    () => {
        deps!();
        fn probe_from_env () -> ProbeResult { let var = | name | env :: var_os (name) . map (PathBuf :: from) . filter (| p | p . exists ()) ; ProbeResult { cert_file : var (ENV_CERT_FILE) , cert_dir : var (ENV_CERT_DIR) , } }
    };
}

probe_from_env!()