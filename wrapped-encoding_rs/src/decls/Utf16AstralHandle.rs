macro_rules! deps {
    () => {
        Utf16Destination!();
    };
}

macro_rules! Utf16AstralHandle {
    () => {
        deps!();
        pub struct Utf16AstralHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut Utf16Destination < 'b > , }
    };
}

Utf16AstralHandle!();