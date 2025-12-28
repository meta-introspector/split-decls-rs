macro_rules! deps {
    () => {
        Number!();
        Result!();
        IndexStr!();
        NvOffset!();
        ParseContext!();
        Parse!();
        SubstitutionTable!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl Parse for NvOffset { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (NvOffset , IndexStr < 'b >) > { try_begin_parse ! ("NvOffset" , ctx , input) ; Number :: parse (ctx , subs , input) . map (| (num , tail) | (NvOffset (num) , tail)) } }
    };
}

impl_147!()