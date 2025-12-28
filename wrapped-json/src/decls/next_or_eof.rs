macro_rules! deps {
    () => {
        Result!();
        Read!();
        ErrorCode!();
    };
}

macro_rules! next_or_eof {
    () => {
        deps!();
        fn next_or_eof < 'de , R > (read : & mut R) -> Result < u8 > where R : ? Sized + Read < 'de > , { match tri ! (read . next ()) { Some (b) => Ok (b) , None => error (read , ErrorCode :: EofWhileParsingString) , } }
    };
}

next_or_eof!()