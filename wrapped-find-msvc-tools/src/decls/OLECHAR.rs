macro_rules! deps {
    () => {
        WCHAR!();
    };
}

macro_rules! OLECHAR {
    () => {
        deps!();
        pub type OLECHAR = WCHAR ;
    };
}

OLECHAR!()