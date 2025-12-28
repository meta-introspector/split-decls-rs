macro_rules! deps {
    () => {
        IndexerProgress!();
        OdbPackwriterCb!();
        Error!();
        Progress!();
        OdbPackwriter!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl < 'repo > OdbPackwriter < 'repo > { # [doc = " Finish writing the packfile"] pub fn commit (& mut self) -> Result < i32 , Error > { unsafe { let writepack = & * self . raw ; let res = match writepack . commit { Some (commit) => commit (self . raw , & mut self . progress) , None => - 1 , } ; if res < 0 { Err (Error :: last_error (res)) } else { Ok (res) } } } # [doc = " The callback through which progress is monitored. Be aware that this is"] # [doc = " called inline, so performance may be affected."] pub fn progress < F > (& mut self , cb : F) -> & mut OdbPackwriter < 'repo > where F : FnMut (Progress < '_ >) -> bool + 'repo , { let progress_payload = unsafe { & mut * (self . progress_payload_ptr as * mut OdbPackwriterCb < '_ >) } ; progress_payload . cb = Some (Box :: new (cb) as Box < IndexerProgress < 'repo > >) ; self } }
    };
}

impl_509!()