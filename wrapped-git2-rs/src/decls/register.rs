macro_rules! deps {
    () => {
        Transport!();
        Error!();
        TransportData!();
        Remote!();
    };
}

macro_rules! register {
    () => {
        deps!();
        # [doc = " Add a custom transport definition, to be used in addition to the built-in"] # [doc = " set of transports that come with libgit2."] # [doc = ""] # [doc = " This function is unsafe as it needs to be externally synchronized with calls"] # [doc = " to creation of other transports."] pub unsafe fn register < F > (prefix : & str , factory : F) -> Result < () , Error > where F : Fn (& Remote < '_ >) -> Result < Transport , Error > + Send + Sync + 'static , { crate :: init () ; let mut data = Box :: new (TransportData { factory : Box :: new (factory) , }) ; let prefix = CString :: new (prefix) ? ; let datap = (& mut * data) as * mut TransportData as * mut c_void ; let factory : raw :: git_transport_cb = Some (transport_factory) ; try_call ! (raw :: git_transport_register (prefix , factory , datap)) ; mem :: forget (data) ; Ok (()) }
    };
}

register!();