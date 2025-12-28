macro_rules! deps {
    () => {
        FetchPrune!();
        ProxyOptions!();
        RemoteCallbacks!();
        FetchOptions!();
        RemoteRedirect!();
        AutotagOption!();
    };
}

macro_rules! impl_654 {
    () => {
        deps!();
        impl < 'cb > FetchOptions < 'cb > { # [doc = " Creates a new blank set of fetch options"] pub fn new () -> FetchOptions < 'cb > { FetchOptions { callbacks : None , proxy : None , prune : FetchPrune :: Unspecified , update_flags : RemoteUpdateFlags :: UPDATE_FETCHHEAD , download_tags : AutotagOption :: Unspecified , follow_redirects : RemoteRedirect :: Initial , custom_headers : Vec :: new () , custom_headers_ptrs : Vec :: new () , depth : 0 , } } # [doc = " Set the callbacks to use for the fetch operation."] pub fn remote_callbacks (& mut self , cbs : RemoteCallbacks < 'cb >) -> & mut Self { self . callbacks = Some (cbs) ; self } # [doc = " Set the proxy options to use for the fetch operation."] pub fn proxy_options (& mut self , opts : ProxyOptions < 'cb >) -> & mut Self { self . proxy = Some (opts) ; self } # [doc = " Set whether to perform a prune after the fetch."] pub fn prune (& mut self , prune : FetchPrune) -> & mut Self { self . prune = prune ; self } # [doc = " Set whether to write the results to FETCH_HEAD."] # [doc = ""] # [doc = " Defaults to `true`."] pub fn update_fetchhead (& mut self , update : bool) -> & mut Self { self . update_flags . set (RemoteUpdateFlags :: UPDATE_FETCHHEAD , update) ; self } # [doc = " Set whether to report unchanged tips in the update_tips callback."] # [doc = ""] # [doc = " Defaults to `false`."] pub fn report_unchanged (& mut self , update : bool) -> & mut Self { self . update_flags . set (RemoteUpdateFlags :: REPORT_UNCHANGED , update) ; self } # [doc = " Set fetch depth, a value less or equal to 0 is interpreted as pull"] # [doc = " everything (effectively the same as not declaring a limit depth)."] pub fn depth (& mut self , depth : i32) -> & mut Self { self . depth = depth . max (0) ; self } # [doc = " Set how to behave regarding tags on the remote, such as auto-downloading"] # [doc = " tags for objects we're downloading or downloading all of them."] # [doc = ""] # [doc = " The default is to auto-follow tags."] pub fn download_tags (& mut self , opt : AutotagOption) -> & mut Self { self . download_tags = opt ; self } # [doc = " Set remote redirection settings; whether redirects to another host are"] # [doc = " permitted."] # [doc = ""] # [doc = " By default, git will follow a redirect on the initial request"] # [doc = " (`/info/refs`), but not subsequent requests."] pub fn follow_redirects (& mut self , redirect : RemoteRedirect) -> & mut Self { self . follow_redirects = redirect ; self } # [doc = " Set extra headers for this fetch operation."] pub fn custom_headers (& mut self , custom_headers : & [& str]) -> & mut Self { self . custom_headers = custom_headers . iter () . map (| & s | CString :: new (s) . unwrap ()) . collect () ; self . custom_headers_ptrs = self . custom_headers . iter () . map (| s | s . as_ptr ()) . collect () ; self } }
    };
}

impl_654!()