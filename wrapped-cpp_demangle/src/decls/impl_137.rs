macro_rules! deps {
    () => {
        IndexStr!();
        Parse!();
        Result!();
        SeqId!();
        SubstitutionTable!();
        ParseContext!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl Parse for SeqId { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (SeqId , IndexStr < 'b >) > { try_begin_parse ! ("SeqId" , ctx , input) ; parse_number (36 , false , input) . map (| (num , tail) | (SeqId (num as _) , tail)) } }
    };
}

impl_137!()