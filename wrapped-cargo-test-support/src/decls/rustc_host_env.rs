macro_rules! rustc_host_env {
    () => {
        # [doc = " The host triple suitable for use in a cargo environment variable (uppercased)."] pub fn rustc_host_env () -> String { rustc_host () . to_uppercase () . replace ('-' , "_") }
    };
}

rustc_host_env!();