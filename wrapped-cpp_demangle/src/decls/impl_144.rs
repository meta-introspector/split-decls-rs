macro_rules! deps {
    () => {
        NvOffset!();
        Parse!();
        IndexStr!();
        Error!();
        ParseContext!();
        CallOffset!();
        Result!();
        SubstitutionTable!();
        VOffset!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Parse for CallOffset { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (CallOffset , IndexStr < 'b >) > { try_begin_parse ! ("CallOffset" , ctx , input) ; if input . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } if let Ok (tail) = consume (b"h" , input) { let (offset , tail) = NvOffset :: parse (ctx , subs , tail) ? ; let tail = consume (b"_" , tail) ? ; return Ok ((CallOffset :: NonVirtual (offset) , tail)) ; } if let Ok (tail) = consume (b"v" , input) { let (offset , tail) = VOffset :: parse (ctx , subs , tail) ? ; let tail = consume (b"_" , tail) ? ; return Ok ((CallOffset :: Virtual (offset) , tail)) ; } Err (error :: Error :: UnexpectedText) } }
    };
}

impl_144!()