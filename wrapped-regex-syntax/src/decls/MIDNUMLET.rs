macro_rules! MIDNUMLET {
    () => {
        pub const MIDNUMLET : & 'static [(char , char)] = & [('.' , '.') , ('‘' , '’') , ('․' , '․') , ('﹒' , '﹒') , ('＇' , '＇') , ('．' , '．') ,] ;
    };
}

MIDNUMLET!();