macro_rules! deps {
    () => {
        Parse!();
        Result!();
        ParseContext!();
        SubstitutionTable!();
        IndexStr!();
        CloneSuffix!();
        CloneTypeIdentifier!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Parse for CloneSuffix { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (CloneSuffix , IndexStr < 'b >) > { try_begin_parse ! ("CloneSuffix" , ctx , input) ; let tail = consume (b"." , input) ? ; let (identifier , mut tail) = CloneTypeIdentifier :: parse (ctx , subs , tail) ? ; let mut numbers = Vec :: with_capacity (1) ; while let Ok ((n , t)) = consume (b"." , tail) . and_then (| t | parse_number (10 , false , t)) { numbers . push (n) ; tail = t ; } let clone_suffix = CloneSuffix (identifier , numbers) ; Ok ((clone_suffix , tail)) } }
    };
}

impl_73!();