macro_rules! deps {
    () => {
        RemoteCallbacks!();
    };
}

macro_rules! sideband_progress_cb {
    () => {
        deps!();
        extern "C" fn sideband_progress_cb (str : * const c_char , len : c_int , payload : * mut c_void) -> c_int { let ok = panic :: wrap (| | unsafe { let payload = & mut * (payload as * mut RemoteCallbacks < '_ >) ; let callback = match payload . sideband_progress { Some (ref mut c) => c , None => return true , } ; let buf = slice :: from_raw_parts (str as * const u8 , len as usize) ; callback (buf) }) ; if ok == Some (true) { 0 } else { - 1 } }
    };
}

sideband_progress_cb!()