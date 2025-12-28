macro_rules! deps {
    () => {
        SubstitutionTable!();
        ParseContext!();
        Parse!();
        IndexStr!();
        Error!();
        Identifier!();
        Result!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Parse for Identifier { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Identifier , IndexStr < 'b >) > { try_begin_parse ! ("Identifier" , ctx , input) ; if input . is_empty () { return Err (error :: Error :: UnexpectedEnd) ; } let end = input . as_ref () . iter () . map (| & c | c as char) . take_while (| & c | c == '$' || c == '_' || c == '.' || c . is_digit (36)) . count () ; if end == 0 { return Err (error :: Error :: UnexpectedText) ; } let tail = input . range_from (end ..) ; let identifier = Identifier { start : input . index () , end : tail . index () , } ; Ok ((identifier , tail)) } }
    };
}

impl_129!()