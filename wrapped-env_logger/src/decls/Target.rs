macro_rules! Target {
    () => {
        # [doc = " Log target, either `stdout`, `stderr` or a custom pipe."] # [non_exhaustive] # [derive (Default)] pub enum Target { # [doc = " Logs will be sent to standard output."] Stdout , # [doc = " Logs will be sent to standard error."] # [default] Stderr , # [doc = " Logs will be sent to a custom pipe."] Pipe (Box < dyn std :: io :: Write + Send + 'static >) , }
    };
}

Target!();