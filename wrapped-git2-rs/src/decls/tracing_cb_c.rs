macro_rules! deps {
    () => {
        TraceLevel!();
        TracingCb!();
        Binding!();
    };
}

macro_rules! tracing_cb_c {
    () => {
        deps!();
        # [doc = " The tracing callback we pass to libgit2 (C ABI compatible)."] extern "C" fn tracing_cb_c (level : raw :: git_trace_level_t , msg : * const c_char) { let cb : * mut () = CALLBACK . load (Ordering :: SeqCst) ; let cb : TracingCb = unsafe { std :: mem :: transmute (cb) } ; if msg . is_null () { return ; } let msg : & CStr = unsafe { CStr :: from_ptr (msg) } ; let msg : & [u8] = CStr :: to_bytes (msg) ; let level : TraceLevel = unsafe { Binding :: from_raw (level) } ; (cb) (level , msg) ; }
    };
}

tracing_cb_c!()