macro_rules! deps {
    () => {
        InvalidUuid!();
    };
}

macro_rules! try_parse {
    () => {
        deps!();
        # [inline] pub const fn try_parse (input : & '_ str) -> Result < [u8 ; 16] , InvalidUuid < '_ > > { let result = match (input . len () , input . as_bytes ()) { (32 , s) => parse_simple (s) , (36 , s) | (38 , [b'{' , s @ .. , b'}']) | (45 , [b'u' , b'r' , b'n' , b':' , b'u' , b'u' , b'i' , b'd' , b':' , s @ ..]) => { parse_hyphenated (s) } _ => Err (()) , } ; match result { Ok (b) => Ok (b) , Err (()) => Err (InvalidUuid (input)) , } }
    };
}

try_parse!()