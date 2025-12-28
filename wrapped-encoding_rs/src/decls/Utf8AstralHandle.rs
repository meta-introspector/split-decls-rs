macro_rules! deps {
    () => {
        Utf8Destination!();
    };
}

macro_rules! Utf8AstralHandle {
    () => {
        deps!();
        pub struct Utf8AstralHandle < 'a , 'b > where 'b : 'a , { dest : & 'a mut Utf8Destination < 'b > , }
    };
}

Utf8AstralHandle!();