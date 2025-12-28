macro_rules! deps {
    () => {
        TemplateArg!();
        Parse!();
        TemplateArgs!();
        ParseContext!();
        IndexStr!();
        Result!();
        SubstitutionTable!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl Parse for TemplateArgs { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (TemplateArgs , IndexStr < 'b >) > { try_begin_parse ! ("TemplateArgs" , ctx , input) ; let tail = consume (b"I" , input) ? ; let (args , tail) = one_or_more :: < TemplateArg > (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; Ok ((TemplateArgs (args) , tail)) } }
    };
}

impl_233!();