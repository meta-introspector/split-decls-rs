macro_rules! QUOTATION_MARK {
    () => {
        pub const QUOTATION_MARK : & 'static [(char , char)] = & [('"' , '"') , ('\'' , '\'') , ('«' , '«') , ('»' , '»') , ('‘' , '‟') , ('‹' , '›') , ('⹂' , '⹂') , ('「' , '』') , ('〝' , '〟') , ('﹁' , '﹄') , ('＂' , '＂') , ('＇' , '＇') , ('｢' , '｣') ,] ;
    };
}

QUOTATION_MARK!()