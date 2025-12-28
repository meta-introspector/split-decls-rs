macro_rules! ProxyOptions {
    () => {
        # [doc = " Options which can be specified to various fetch operations."] # [derive (Default)] pub struct ProxyOptions < 'a > { url : Option < CString > , proxy_kind : raw :: git_proxy_t , _marker : marker :: PhantomData < & 'a i32 > , }
    };
}

ProxyOptions!()