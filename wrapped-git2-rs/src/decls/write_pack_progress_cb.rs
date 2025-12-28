macro_rules! deps {
    () => {
        Binding!();
        OdbPackwriterCb!();
        Progress!();
    };
}

macro_rules! write_pack_progress_cb {
    () => {
        deps!();
        pub (crate) extern "C" fn write_pack_progress_cb (stats : * const raw :: git_indexer_progress , payload : * mut c_void ,) -> c_int { let ok = panic :: wrap (| | unsafe { let payload = & mut * (payload as * mut OdbPackwriterCb < '_ >) ; let callback = match payload . cb { Some (ref mut cb) => cb , None => return true , } ; let progress : Progress < '_ > = Binding :: from_raw (stats) ; callback (progress) }) ; if ok == Some (true) { 0 } else { - 1 } }
    };
}

write_pack_progress_cb!()