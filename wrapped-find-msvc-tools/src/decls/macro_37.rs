macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! macro_37 {
    () => {
        deps!();
        windows_link :: link ! ("oleaut32.dll" "system" fn SysStringLen (pbstr : BSTR) -> u32) ;
    };
}

macro_37!()