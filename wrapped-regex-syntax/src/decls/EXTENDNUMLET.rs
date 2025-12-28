macro_rules! EXTENDNUMLET {
    () => {
        pub const EXTENDNUMLET : & 'static [(char , char)] = & [('_' , '_') , ('\u{202f}' , '\u{202f}') , ('‿' , '⁀') , ('⁔' , '⁔') , ('︳' , '︴') , ('﹍' , '﹏') , ('＿' , '＿') ,] ;
    };
}

EXTENDNUMLET!()