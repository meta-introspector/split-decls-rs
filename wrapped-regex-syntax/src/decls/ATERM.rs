macro_rules! ATERM {
    () => {
        pub const ATERM : & 'static [(char , char)] = & [('.' , '.') , ('․' , '․') , ('﹒' , '﹒') , ('．' , '．')] ;
    };
}

ATERM!()