macro_rules! deps {
    () => {
        Utf8Destination!();
    };
}

macro_rules! Utf8BmpHandle {
    () => {
        deps!();
        pub struct Utf8BmpHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut Utf8Destination < 'b > , }
    };
}

Utf8BmpHandle!()