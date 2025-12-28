macro_rules! deps {
    () => {
        Expression!();
        Result!();
        Error!();
        Parse!();
        IndexStr!();
        ParametricBuiltinType!();
        SubstitutionTable!();
        ParseContext!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl Parse for ParametricBuiltinType { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (ParametricBuiltinType , IndexStr < 'b >) > { try_begin_parse ! ("ParametricBuiltinType" , ctx , input) ; let input = consume (b"D" , input) ? ; let (ch , input) = input . next_or (error :: Error :: UnexpectedEnd) ? ; let allow_expression = match ch { b'F' => false , b'B' | b'U' => true , _ => return Err (error :: Error :: UnexpectedText) , } ; if input . next_or (error :: Error :: UnexpectedEnd) ? . 0 . is_ascii_digit () { let (bit_size , input) = parse_number (10 , false , input) ? ; if ch == b'F' { if let Ok (input) = consume (b"x" , input) { return Ok ((ParametricBuiltinType :: FloatNx (bit_size) , input)) ; } } let input = consume (b"_" , input) ? ; let t = match ch { b'F' => ParametricBuiltinType :: FloatN (bit_size) , b'B' => ParametricBuiltinType :: SignedBitInt (bit_size) , b'U' => ParametricBuiltinType :: UnsignedBitInt (bit_size) , _ => panic ! ("oh noes") , } ; Ok ((t , input)) } else if allow_expression { let (expr , input) = Expression :: parse (ctx , subs , input) ? ; let expr = Box :: new (expr) ; let t = match ch { b'B' => ParametricBuiltinType :: SignedBitIntExpression (expr) , b'U' => ParametricBuiltinType :: UnsignedBitIntExpression (expr) , _ => panic ! ("oh noes") , } ; Ok ((t , input)) } else { Err (error :: Error :: UnexpectedText) } } }
    };
}

impl_173!();