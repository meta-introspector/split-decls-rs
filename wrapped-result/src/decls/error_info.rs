macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! error_info {
    () => {
        deps!();
        # [cfg (not (all (windows , not (windows_slim_errors))))] mod error_info { use super :: * ; # [derive (Clone , Default)] pub (crate) struct EmptyErrorInfo ; pub (crate) use EmptyErrorInfo as ErrorInfo ; impl EmptyErrorInfo { pub (crate) const fn empty () -> Self { Self } pub (crate) fn from_thread () -> Self { Self } pub (crate) fn into_thread (self) { } # [cfg (windows)] pub (crate) fn originate_error (_code : HRESULT , _message : & str) { } pub (crate) fn message (& self) -> Option < String > { None } # [cfg (windows)] pub (crate) fn as_ptr (& self) -> * mut core :: ffi :: c_void { core :: ptr :: null_mut () } } }
    };
}

error_info!()