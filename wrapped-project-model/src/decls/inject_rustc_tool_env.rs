macro_rules! deps {
    () => {
        TargetKind!();
    };
}

macro_rules! inject_rustc_tool_env {
    () => {
        deps!();
        pub (crate) fn inject_rustc_tool_env (env : & mut Env , cargo_name : & str , kind : TargetKind) { _ = kind ; env . set ("CARGO_CRATE_NAME" , cargo_name . replace ('-' , "_")) ; }
    };
}

inject_rustc_tool_env!()