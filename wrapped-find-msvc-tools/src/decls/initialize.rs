macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! initialize {
    () => {
        deps!();
        pub fn initialize () -> Result < () , HRESULT > { let err = unsafe { CoInitializeEx (null () , COINIT_MULTITHREADED . try_into () . unwrap ()) } ; if err != S_OK && err != S_FALSE { Err (err) } else { Ok (()) } }
    };
}

initialize!();