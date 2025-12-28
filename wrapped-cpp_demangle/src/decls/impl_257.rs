macro_rules! deps {
    () => {
        SubstitutionTable!();
        SourceName!();
        IndexStr!();
        Parse!();
        ParseContext!();
        TemplateArgs!();
        SimpleId!();
        Result!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl Parse for SimpleId { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (SimpleId , IndexStr < 'b >) > { try_begin_parse ! ("SimpleId" , ctx , input) ; let (name , tail) = SourceName :: parse (ctx , subs , input) ? ; let (args , tail) = if let Ok ((args , tail)) = try_recurse ! (TemplateArgs :: parse (ctx , subs , tail)) { (Some (args) , tail) } else { (None , tail) } ; Ok ((SimpleId (name , args) , tail)) } }
    };
}

impl_257!()