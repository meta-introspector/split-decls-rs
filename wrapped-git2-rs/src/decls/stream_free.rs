macro_rules! deps {
    () => {
        RawSmartSubtransportStream!();
    };
}

macro_rules! stream_free {
    () => {
        deps!();
        extern "C" fn stream_free (stream : * mut raw :: git_smart_subtransport_stream) { let _ = panic :: wrap (| | unsafe { mem :: transmute :: < _ , Box < RawSmartSubtransportStream > > (stream) ; }) ; }
    };
}

stream_free!()