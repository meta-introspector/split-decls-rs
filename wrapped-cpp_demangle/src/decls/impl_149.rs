macro_rules! deps {
    () => {
        IndexStr!();
        VOffset!();
        Result!();
        SubstitutionTable!();
        Number!();
        Parse!();
        ParseContext!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Parse for VOffset { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (VOffset , IndexStr < 'b >) > { try_begin_parse ! ("VOffset" , ctx , input) ; let (offset , tail) = Number :: parse (ctx , subs , input) ? ; let tail = consume (b"_" , tail) ? ; let (virtual_offset , tail) = Number :: parse (ctx , subs , tail) ? ; Ok ((VOffset (offset , virtual_offset) , tail)) } }
    };
}

impl_149!();