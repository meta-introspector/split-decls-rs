macro_rules! deps {
    () => {
        Read!();
        ErrorCode!();
        Result!();
    };
}

macro_rules! peek_or_eof {
    () => {
        deps!();
        fn peek_or_eof < 'de , R > (read : & mut R) -> Result < u8 > where R : ? Sized + Read < 'de > , { match tri ! (read . peek ()) { Some (b) => Ok (b) , None => error (read , ErrorCode :: EofWhileParsingString) , } }
    };
}

peek_or_eof!()