macro_rules! deps {
    () => {
        ParseContext!();
        IndexStr!();
        Result!();
        SubstitutionTable!();
        Parse!();
        SeqId!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl Parse for SeqId { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (SeqId , IndexStr < 'b >) > { try_begin_parse ! ("SeqId" , ctx , input) ; parse_number (36 , false , input) . map (| (num , tail) | (SeqId (num as _) , tail)) } }
    };
}

impl_137!();