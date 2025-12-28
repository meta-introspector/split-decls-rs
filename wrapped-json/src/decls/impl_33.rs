macro_rules! deps {
    () => {
        MapAccess!();
        Error!();
        Result!();
        MapKey!();
        Value!();
        ErrorCode!();
        Read!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'de , 'a , R : Read < 'de > + 'a > de :: MapAccess < 'de > for MapAccess < 'a , R > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > > where K : de :: DeserializeSeed < 'de > , { fn has_next_key < 'de , 'a , R : Read < 'de > + 'a > (map : & mut MapAccess < 'a , R >) -> Result < bool > { let peek = match tri ! (map . de . parse_whitespace ()) { Some (b) => b , None => { return Err (map . de . peek_error (ErrorCode :: EofWhileParsingObject)) ; } } ; if peek == b'}' { Ok (false) } else if map . first { map . first = false ; if peek == b'"' { Ok (true) } else { Err (map . de . peek_error (ErrorCode :: KeyMustBeAString)) } } else if peek == b',' { map . de . eat_char () ; match tri ! (map . de . parse_whitespace ()) { Some (b'"') => Ok (true) , Some (b'}') => Err (map . de . peek_error (ErrorCode :: TrailingComma)) , Some (_) => Err (map . de . peek_error (ErrorCode :: KeyMustBeAString)) , None => Err (map . de . peek_error (ErrorCode :: EofWhileParsingValue)) , } } else { Err (map . de . peek_error (ErrorCode :: ExpectedObjectCommaOrEnd)) } } if tri ! (has_next_key (self)) { Ok (Some (tri ! (seed . deserialize (MapKey { de : & mut * self . de })))) } else { Ok (None) } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value > where V : de :: DeserializeSeed < 'de > , { tri ! (self . de . parse_object_colon ()) ; seed . deserialize (& mut * self . de) } }
    };
}

impl_33!();