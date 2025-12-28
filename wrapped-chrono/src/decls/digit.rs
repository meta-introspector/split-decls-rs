macro_rules! deps {
    () => {
        ParseResult!();
    };
}

macro_rules! digit {
    () => {
        deps!();
        # [inline] fn digit (bytes : & [u8 ; 19] , index : usize) -> ParseResult < u8 > { match bytes [index] . is_ascii_digit () { true => Ok (bytes [index] - b'0') , false => Err (INVALID) , } }
    };
}

digit!()