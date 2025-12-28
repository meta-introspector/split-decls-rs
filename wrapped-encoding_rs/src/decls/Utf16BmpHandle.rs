macro_rules! deps {
    () => {
        Utf16Destination!();
    };
}

macro_rules! Utf16BmpHandle {
    () => {
        deps!();
        pub struct Utf16BmpHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut Utf16Destination < 'b > , }
    };
}

Utf16BmpHandle!();