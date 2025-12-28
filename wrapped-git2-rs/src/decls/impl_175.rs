macro_rules! deps {
    () => {
        Transport!();
        Error!();
        SmartSubtransport!();
        RawSmartSubtransport!();
        Remote!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl Transport { # [doc = " Creates a new transport which will use the \"smart\" transport protocol"] # [doc = " for transferring data."] # [doc = ""] # [doc = " A smart transport requires a *subtransport* over which data is actually"] # [doc = " communicated, but this subtransport largely just needs to be able to"] # [doc = " read() and write(). The subtransport provided will be used to make"] # [doc = " connections which can then be read/written from."] # [doc = ""] # [doc = " The `rpc` argument is `true` if the protocol is stateless, false"] # [doc = " otherwise. For example `http://` is stateless but `git://` is not."] pub fn smart < S > (remote : & Remote < '_ > , rpc : bool , subtransport : S) -> Result < Transport , Error > where S : SmartSubtransport , { let mut ret = ptr :: null_mut () ; let mut raw = Box :: new (RawSmartSubtransport { raw : raw :: git_smart_subtransport { action : Some (subtransport_action) , close : Some (subtransport_close) , free : Some (subtransport_free) , } , stream : None , rpc , obj : Box :: new (subtransport) , }) ; let mut defn = raw :: git_smart_subtransport_definition { callback : Some (smart_factory) , rpc : rpc as c_uint , param : & mut * raw as * mut _ as * mut _ , } ; unsafe { try_call ! (raw :: git_transport_smart (& mut ret , remote . raw () , & mut defn as * mut _ as * mut _)) ; mem :: forget (raw) ; } return Ok (Transport { raw : ret , owned : true , }) ; extern "C" fn smart_factory (out : * mut * mut raw :: git_smart_subtransport , _owner : * mut raw :: git_transport , ptr : * mut c_void ,) -> c_int { unsafe { * out = ptr as * mut raw :: git_smart_subtransport ; 0 } } } }
    };
}

impl_175!();