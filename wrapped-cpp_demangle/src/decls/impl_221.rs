macro_rules! deps {
    () => {
        Parse!();
        ParseContext!();
        IndexStr!();
        TemplateParam!();
        Result!();
        SubstitutionTable!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl Parse for TemplateParam { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (TemplateParam , IndexStr < 'b >) > { try_begin_parse ! ("TemplateParam" , ctx , input) ; let input = consume (b"T" , input) ? ; let (number , input) = match parse_number (10 , false , input) { Ok ((number , input)) => ((number + 1) as _ , input) , Err (_) => (0 , input) , } ; let input = consume (b"_" , input) ? ; Ok ((TemplateParam (number) , input)) } }
    };
}

impl_221!()