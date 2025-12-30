// Generated macro for get_cert_path (function)
macro_rules! Depcrateget_cert_path {
() => {
// Module: crate
// Provides: {"get_cert_path"}
// Dependencies: {}
# [doc = " Returns the path to the X.509 certificate and key."] # [doc = ""] # [doc = " If `QUICHE_FUZZ_CRT` and / or `QUICHE_FUZZ_KEY` are set, their value is"] # [doc = " used, otherwise in order to accomodate different fuzzing environments, this"] # [doc = " either returns relative paths (e.g. \"fuzz/cert.crt\") when running from a"] # [doc = " clone of the git repository, or absolute paths based on `argv[0]` when"] # [doc = " running bare executable (as used by OSS-Fuzz)."] pub fn get_cert_path () -> (String , String) { let fuzz_dir = if Path :: new ("fuzz/") . exists () { PathBuf :: from ("fuzz/") } else { let mut fuzz_dir = PathBuf :: from (std :: env :: args () . next () . unwrap ()) ; fuzz_dir . pop () ; fuzz_dir . push ("fuzz") ; fuzz_dir } ; let crt_path = std :: env :: var ("QUICHE_FUZZ_CRT") . unwrap_or_else (| _ | { let mut crt_path = fuzz_dir . clone () ; crt_path . push ("cert.crt") ; crt_path . to_str () . unwrap () . to_string () }) ; let key_path = std :: env :: var ("QUICHE_FUZZ_KEY") . unwrap_or_else (| _ | { let mut key_path = fuzz_dir . clone () ; key_path . push ("cert.key") ; key_path . to_str () . unwrap () . to_string () }) ; (crt_path , key_path) }
};
}
