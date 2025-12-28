macro_rules! deps {
    () => {
        Utf16Source!();
    };
}

macro_rules! Utf16ReadHandle {
    () => {
        deps!();
        pub struct Utf16ReadHandle < 'a , 'b > where 'b : 'a , { source : & 'a mut Utf16Source < 'b > , }
    };
}

Utf16ReadHandle!();