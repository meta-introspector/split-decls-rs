macro_rules! deps {
    () => {
        SubstitutionTable!();
        IndexStr!();
        Parse!();
        ParseContext!();
        Result!();
        Error!();
        Discriminator!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl Parse for Discriminator { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Discriminator , IndexStr < 'b >) > { try_begin_parse ! ("Discriminator" , ctx , input) ; let tail = consume (b"_" , input) ? ; if let Ok (tail) = consume (b"_" , tail) { let (num , tail) = parse_number (10 , false , tail) ? ; debug_assert ! (num >= 0) ; if num < 10 { return Err (error :: Error :: UnexpectedText) ; } let tail = consume (b"_" , tail) ? ; return Ok ((Discriminator (num as _) , tail)) ; } match tail . try_split_at (1) { None => Err (error :: Error :: UnexpectedEnd) , Some ((head , tail)) => match head . as_ref () [0] { b'0' => Ok ((Discriminator (0) , tail)) , b'1' => Ok ((Discriminator (1) , tail)) , b'2' => Ok ((Discriminator (2) , tail)) , b'3' => Ok ((Discriminator (3) , tail)) , b'4' => Ok ((Discriminator (4) , tail)) , b'5' => Ok ((Discriminator (5) , tail)) , b'6' => Ok ((Discriminator (6) , tail)) , b'7' => Ok ((Discriminator (7) , tail)) , b'8' => Ok ((Discriminator (8) , tail)) , b'9' => Ok ((Discriminator (9) , tail)) , _ => Err (error :: Error :: UnexpectedText) , } , } } }
    };
}

impl_277!();