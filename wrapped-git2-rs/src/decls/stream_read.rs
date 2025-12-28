macro_rules! deps {
    () => {
        RawSmartSubtransportStream!();
    };
}

macro_rules! stream_read {
    () => {
        deps!();
        extern "C" fn stream_read (stream : * mut raw :: git_smart_subtransport_stream , buffer : * mut c_char , buf_size : size_t , bytes_read : * mut size_t ,) -> c_int { let ret = panic :: wrap (| | unsafe { let transport = & mut * (stream as * mut RawSmartSubtransportStream) ; let buf = slice :: from_raw_parts_mut (buffer as * mut u8 , buf_size as usize) ; match transport . obj . read (buf) { Ok (n) => { * bytes_read = n as size_t ; Ok (n) } e => e , } }) ; match ret { Some (Ok (_)) => 0 , Some (Err (e)) => unsafe { set_err_io (& e) ; - 2 } , None => - 1 , } }
    };
}

stream_read!()