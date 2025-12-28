macro_rules! CONNECTOR_PUNCTUATION {
    () => {
        pub const CONNECTOR_PUNCTUATION : & 'static [(char , char)] = & [('_' , '_') , ('‿' , '⁀') , ('⁔' , '⁔') , ('︳' , '︴') , ('﹍' , '﹏') , ('＿' , '＿') ,] ;
    };
}

CONNECTOR_PUNCTUATION!();