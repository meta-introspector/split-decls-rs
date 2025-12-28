macro_rules! deps {
    () => {
        Discriminator!();
        Name!();
        Parse!();
        Encoding!();
        IndexStr!();
        Number!();
        Result!();
        ParseContext!();
        SubstitutionTable!();
        LocalName!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl Parse for LocalName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (LocalName , IndexStr < 'b >) > { try_begin_parse ! ("LocalName" , ctx , input) ; let tail = consume (b"Z" , input) ? ; let (encoding , tail) = Encoding :: parse (ctx , subs , tail) ? ; let tail = consume (b"E" , tail) ? ; if let Ok (tail) = consume (b"s" , tail) { let (disc , tail) = if let Ok ((disc , tail)) = try_recurse ! (Discriminator :: parse (ctx , subs , tail)) { (Some (disc) , tail) } else { (None , tail) } ; return Ok ((LocalName :: Relative (Box :: new (encoding) , None , disc) , tail)) ; } if let Ok (tail) = consume (b"d" , tail) { let (param , tail) = if let Ok ((num , tail)) = try_recurse ! (Number :: parse (ctx , subs , tail)) { (Some (num as _) , tail) } else { (None , tail) } ; let tail = consume (b"_" , tail) ? ; let (name , tail) = Name :: parse (ctx , subs , tail) ? ; return Ok ((LocalName :: Default (Box :: new (encoding) , param , Box :: new (name)) , tail ,)) ; } let (name , tail) = Name :: parse (ctx , subs , tail) ? ; let (disc , tail) = if let Ok ((disc , tail)) = try_recurse ! (Discriminator :: parse (ctx , subs , tail)) { (Some (disc) , tail) } else { (None , tail) } ; Ok ((LocalName :: Relative (Box :: new (encoding) , Some (Box :: new (name)) , disc) , tail ,)) } }
    };
}

impl_272!();