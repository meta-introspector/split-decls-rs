macro_rules! deps {
    () => {
        RawSmartSubtransport!();
    };
}

macro_rules! subtransport_close {
    () => {
        deps!();
        extern "C" fn subtransport_close (transport : * mut raw :: git_smart_subtransport) -> c_int { let ret = panic :: wrap (| | unsafe { let transport = & mut * (transport as * mut RawSmartSubtransport) ; transport . obj . close () }) ; match ret { Some (Ok (())) => 0 , Some (Err (e)) => e . raw_code () as c_int , None => - 1 , } }
    };
}

subtransport_close!();