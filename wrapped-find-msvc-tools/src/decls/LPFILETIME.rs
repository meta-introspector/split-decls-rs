macro_rules! deps {
    () => {
        FILETIME!();
    };
}

macro_rules! LPFILETIME {
    () => {
        deps!();
        pub type LPFILETIME = * mut FILETIME ;
    };
}

LPFILETIME!();