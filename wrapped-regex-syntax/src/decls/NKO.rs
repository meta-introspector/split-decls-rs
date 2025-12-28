macro_rules! NKO {
    () => {
        pub const NKO : & 'static [(char , char)] = & [('،' , '،') , ('؛' , '؛') , ('؟' , '؟') , ('߀' , 'ߺ') , ('\u{7fd}' , '߿') , ('﴾' , '﴿') ,] ;
    };
}

NKO!();