macro_rules! LF {
    () => {
        pub const LF : & 'static [(char , char)] = & [('\n' , '\n')] ;
    };
}

LF!()