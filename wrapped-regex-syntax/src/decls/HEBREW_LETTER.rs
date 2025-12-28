macro_rules! HEBREW_LETTER {
    () => {
        pub const HEBREW_LETTER : & 'static [(char , char)] = & [('א' , 'ת') , ('ׯ' , 'ײ') , ('יִ' , 'יִ') , ('ײַ' , 'ﬨ') , ('שׁ' , 'זּ') , ('טּ' , 'לּ') , ('מּ' , 'מּ') , ('נּ' , 'סּ') , ('ףּ' , 'פּ') , ('צּ' , 'ﭏ') ,] ;
    };
}

HEBREW_LETTER!()