macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! macro_9 {
    () => {
        deps!();
        windows_link :: link ! ("oleaut32.dll" "system" fn SysFreeString (bstrstring : BSTR)) ;
    };
}

macro_9!()