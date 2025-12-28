macro_rules! deps {
    () => {
        IndexStr!();
        ParseContext!();
        Parse!();
        Result!();
        SubstitutionTable!();
        Number!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl Parse for Number { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (isize , IndexStr < 'b >) > { try_begin_parse ! ("Number" , ctx , input) ; parse_number (10 , true , input) } }
    };
}

impl_135!();