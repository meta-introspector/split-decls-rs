macro_rules! is_simple_exit_code {
    () => {
        # [doc = " Returns `true` if the given process exit code is something a normal"] # [doc = " process would exit with."] # [doc = ""] # [doc = " This helps differentiate from abnormal termination codes, such as"] # [doc = " segmentation faults or signals."] pub fn is_simple_exit_code (code : i32) -> bool { code >= 0 && code <= 127 }
    };
}

is_simple_exit_code!()