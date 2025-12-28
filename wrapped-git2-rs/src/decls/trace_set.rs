macro_rules! deps {
    () => {
        TraceLevel!();
        TracingCb!();
        Error!();
    };
}

macro_rules! trace_set {
    () => {
        deps!();
        # [doc = " Set the global subscriber called when libgit2 produces a tracing message."] pub fn trace_set (level : TraceLevel , cb : TracingCb) -> Result < () , Error > { CALLBACK . store (cb as * mut () , Ordering :: SeqCst) ; let return_code : c_int = unsafe { raw :: git_trace_set (level . raw () , Some (tracing_cb_c)) } ; if return_code != 0 { Err (Error :: last_error (return_code)) } else { Ok (()) } }
    };
}

trace_set!();