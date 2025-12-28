macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! LineRange {
    () => {
        deps!();
        pub (crate) trait LineRange { fn shift_by (& self , offset : Offset) -> Self ; }
    };
}

LineRange!()