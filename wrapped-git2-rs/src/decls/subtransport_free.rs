macro_rules! deps {
    () => {
        RawSmartSubtransport!();
    };
}

macro_rules! subtransport_free {
    () => {
        deps!();
        extern "C" fn subtransport_free (transport : * mut raw :: git_smart_subtransport) { let _ = panic :: wrap (| | unsafe { mem :: transmute :: < _ , Box < RawSmartSubtransport > > (transport) ; }) ; }
    };
}

subtransport_free!();