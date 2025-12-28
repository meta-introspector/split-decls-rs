macro_rules! DOUBLE_QUOTE {
    () => {
        pub const DOUBLE_QUOTE : & 'static [(char , char)] = & [('"' , '"')] ;
    };
}

DOUBLE_QUOTE!()