macro_rules! deps {
    () => {
        BSTR!();
    };
}

macro_rules! BStr {
    () => {
        deps!();
        pub struct BStr (BSTR) ;
    };
}

BStr!();