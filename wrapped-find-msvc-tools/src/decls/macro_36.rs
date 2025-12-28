macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! macro_36 {
    () => {
        deps!();
        windows_link :: link ! ("oleaut32.dll" "system" fn SysFreeString (bstrstring : BSTR)) ;
    };
}

macro_36!();