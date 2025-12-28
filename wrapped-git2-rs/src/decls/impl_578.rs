macro_rules! deps {
    () => {
        ProxyOptions!();
        Binding!();
    };
}

macro_rules! impl_578 {
    () => {
        deps!();
        impl < 'a > Binding for ProxyOptions < 'a > { type Raw = raw :: git_proxy_options ; unsafe fn from_raw (_raw : raw :: git_proxy_options) -> ProxyOptions < 'a > { panic ! ("can't create proxy from raw options") } fn raw (& self) -> raw :: git_proxy_options { raw :: git_proxy_options { version : raw :: GIT_PROXY_OPTIONS_VERSION , kind : self . proxy_kind , url : self . url . as_ref () . map (| s | s . as_ptr ()) . unwrap_or (ptr :: null ()) , credentials : None , certificate_check : None , payload : ptr :: null_mut () , } } }
    };
}

impl_578!();