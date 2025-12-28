macro_rules! deps {
    () => {
        RemoteCallbacks!();
        Binding!();
    };
}

macro_rules! transfer_progress_cb {
    () => {
        deps!();
        extern "C" fn transfer_progress_cb (stats : * const raw :: git_indexer_progress , payload : * mut c_void ,) -> c_int { let ok = panic :: wrap (| | unsafe { let payload = & mut * (payload as * mut RemoteCallbacks < '_ >) ; let callback = match payload . progress { Some (ref mut c) => c , None => return true , } ; let progress = Binding :: from_raw (stats) ; callback (progress) }) ; if ok == Some (true) { 0 } else { - 1 } }
    };
}

transfer_progress_cb!();