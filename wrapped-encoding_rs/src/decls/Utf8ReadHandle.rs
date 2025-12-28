macro_rules! deps {
    () => {
        Utf8Source!();
    };
}

macro_rules! Utf8ReadHandle {
    () => {
        deps!();
        pub struct Utf8ReadHandle < 'a , 'b > where 'b : 'a , { source : & 'a mut Utf8Source < 'b > , }
    };
}

Utf8ReadHandle!();