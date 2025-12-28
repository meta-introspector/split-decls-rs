macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! win32_error {
    () => {
        deps!();
        fn win32_error (result : u32) -> Result < () > { if result == 0 { Ok (()) } else { Err (Error :: from_hresult (WIN32_ERROR (result) . to_hresult ())) } }
    };
}

win32_error!()