macro_rules! deps {
    () => {
        PushOptions!();
        RemoteRedirect!();
        ProxyOptions!();
        RemoteCallbacks!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl < 'cb > PushOptions < 'cb > { # [doc = " Creates a new blank set of push options"] pub fn new () -> PushOptions < 'cb > { PushOptions { callbacks : None , proxy : None , pb_parallelism : 1 , follow_redirects : RemoteRedirect :: Initial , custom_headers : Vec :: new () , custom_headers_ptrs : Vec :: new () , remote_push_options : Vec :: new () , remote_push_options_ptrs : Vec :: new () , } } # [doc = " Set the callbacks to use for the push operation."] pub fn remote_callbacks (& mut self , cbs : RemoteCallbacks < 'cb >) -> & mut Self { self . callbacks = Some (cbs) ; self } # [doc = " Set the proxy options to use for the push operation."] pub fn proxy_options (& mut self , opts : ProxyOptions < 'cb >) -> & mut Self { self . proxy = Some (opts) ; self } # [doc = " If the transport being used to push to the remote requires the creation"] # [doc = " of a pack file, this controls the number of worker threads used by the"] # [doc = " packbuilder when creating that pack file to be sent to the remote."] # [doc = ""] # [doc = " if set to 0 the packbuilder will auto-detect the number of threads to"] # [doc = " create, and the default value is 1."] pub fn packbuilder_parallelism (& mut self , parallel : u32) -> & mut Self { self . pb_parallelism = parallel ; self } # [doc = " Set remote redirection settings; whether redirects to another host are"] # [doc = " permitted."] # [doc = ""] # [doc = " By default, git will follow a redirect on the initial request"] # [doc = " (`/info/refs`), but not subsequent requests."] pub fn follow_redirects (& mut self , redirect : RemoteRedirect) -> & mut Self { self . follow_redirects = redirect ; self } # [doc = " Set extra headers for this push operation."] pub fn custom_headers (& mut self , custom_headers : & [& str]) -> & mut Self { self . custom_headers = custom_headers . iter () . map (| & s | CString :: new (s) . unwrap ()) . collect () ; self . custom_headers_ptrs = self . custom_headers . iter () . map (| s | s . as_ptr ()) . collect () ; self } # [doc = " Set \"push options\" to deliver to the remote."] pub fn remote_push_options (& mut self , remote_push_options : & [& str]) -> & mut Self { self . remote_push_options = remote_push_options . iter () . map (| & s | CString :: new (s) . unwrap ()) . collect () ; self . remote_push_options_ptrs = self . remote_push_options . iter () . map (| s | s . as_ptr ()) . collect () ; self } }
    };
}

impl_657!();