macro_rules! deps {
    () => {
        Expression!();
        Encoding!();
        Reader!();
        Result!();
    };
}

macro_rules! parse_data {
    () => {
        deps!();
        fn parse_data < R : Reader > (input : & mut R , encoding : Encoding) -> Result < Expression < R > > { if encoding . version >= 5 { let len = R :: Offset :: from_u64 (input . read_uleb128 () ?) ? ; Ok (Expression (input . split (len) ?)) } else { let len = R :: Offset :: from_u16 (input . read_u16 () ?) ; Ok (Expression (input . split (len) ?)) } }
    };
}

parse_data!();