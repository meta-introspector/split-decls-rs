macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
        InvalidUuid!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 'a > InvalidUuid < 'a > { # [doc = " Converts the lightweight error type into detailed diagnostics."] pub fn into_err (self) -> Error { let (s , offset , simple) = match self . 0 . as_bytes () { [b'{' , s @ .. , b'}'] => (s , 1 , false) , [b'u' , b'r' , b'n' , b':' , b'u' , b'u' , b'i' , b'd' , b':' , s @ ..] => { (s , "urn:uuid:" . len () , false) } s => (s , 0 , true) , } ; let mut hyphen_count = 0 ; let mut group_bounds = [0 ; 4] ; let string = unsafe { std :: str :: from_utf8_unchecked (s) } ; for (index , character) in string . char_indices () { let byte = character as u8 ; if character as u32 - byte as u32 > 0 { return Error (ErrorKind :: Char { character , index : index + offset + 1 , }) ; } else if byte == b'-' { if hyphen_count < 4 { group_bounds [hyphen_count] = index ; } hyphen_count += 1 ; } else if ! byte . is_ascii_hexdigit () { return Error (ErrorKind :: Char { character : byte as char , index : index + offset + 1 , }) ; } } if hyphen_count == 0 && simple { Error (ErrorKind :: SimpleLength { len : s . len () }) } else if hyphen_count != 4 { Error (ErrorKind :: GroupCount { count : hyphen_count + 1 , }) } else { const BLOCK_STARTS : [usize ; 5] = [0 , 9 , 14 , 19 , 24] ; for i in 0 .. 4 { if group_bounds [i] != BLOCK_STARTS [i + 1] - 1 { return Error (ErrorKind :: GroupLength { group : i , len : group_bounds [i] - BLOCK_STARTS [i] , index : offset + BLOCK_STARTS [i] + 1 , }) ; } } Error (ErrorKind :: GroupLength { group : 4 , len : s . len () - BLOCK_STARTS [4] , index : offset + BLOCK_STARTS [4] + 1 , }) } } }
    };
}

impl_3!();