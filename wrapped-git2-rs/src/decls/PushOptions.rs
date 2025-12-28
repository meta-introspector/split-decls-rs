macro_rules! deps {
    () => {
        RemoteCallbacks!();
        ProxyOptions!();
        RemoteRedirect!();
    };
}

macro_rules! PushOptions {
    () => {
        deps!();
        # [doc = " Options to control the behavior of a git push."] pub struct PushOptions < 'cb > { callbacks : Option < RemoteCallbacks < 'cb > > , proxy : Option < ProxyOptions < 'cb > > , pb_parallelism : u32 , follow_redirects : RemoteRedirect , custom_headers : Vec < CString > , custom_headers_ptrs : Vec < * const c_char > , remote_push_options : Vec < CString > , remote_push_options_ptrs : Vec < * const c_char > , }
    };
}

PushOptions!();