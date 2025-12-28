macro_rules! deps {
    () => {
        CONTEXT!();
    };
}

macro_rules! macro_87 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn RtlCaptureContext (contextrecord : * mut CONTEXT)) ;
    };
}

macro_87!();