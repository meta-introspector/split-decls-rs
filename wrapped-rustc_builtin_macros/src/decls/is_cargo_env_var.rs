macro_rules! is_cargo_env_var {
    () => {
        # [doc = " Returns `true` if an environment variable from `env!` is one used by Cargo."] fn is_cargo_env_var (var : & str) -> bool { var . starts_with ("CARGO_") || var . starts_with ("DEP_") || matches ! (var , "OUT_DIR" | "OPT_LEVEL" | "PROFILE" | "HOST" | "TARGET") }
    };
}

is_cargo_env_var!();