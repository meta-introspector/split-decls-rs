macro_rules! deps {
    () => {
        Result!();
        ParseContext!();
        SourceName!();
        Error!();
        Identifier!();
        Parse!();
        SubstitutionTable!();
        IndexStr!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl Parse for SourceName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (SourceName , IndexStr < 'b >) > { try_begin_parse ! ("SourceName" , ctx , input) ; let (source_name_len , input) = parse_number (10 , false , input) ? ; debug_assert ! (source_name_len >= 0) ; if source_name_len == 0 { return Err (error :: Error :: UnexpectedText) ; } let (head , tail) = match input . try_split_at (source_name_len as _) { Some ((head , tail)) => (head , tail) , None => return Err (error :: Error :: UnexpectedEnd) , } ; let (identifier , empty) = Identifier :: parse (ctx , subs , head) ? ; if ! empty . is_empty () { return Err (error :: Error :: UnexpectedText) ; } let source_name = SourceName (identifier) ; Ok ((source_name , tail)) } }
    };
}

impl_118!()