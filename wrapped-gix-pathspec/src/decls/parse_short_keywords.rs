macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! parse_short_keywords {
    () => {
        deps!();
        fn parse_short_keywords (input : & [u8] , cursor : & mut usize) -> Result < MagicSignature , Error > { let unimplemented_chars = b"\"#%&'-',;<=>@_`~" ; let mut signature = MagicSignature :: empty () ; while let Some (& b) = input . get (* cursor) { * cursor += 1 ; signature |= match b { b'/' => MagicSignature :: TOP , b'^' | b'!' => MagicSignature :: EXCLUDE , b':' => break , _ if unimplemented_chars . contains (& b) => { return Err (Error :: Unimplemented { short_keyword : b . into () , }) ; } _ => { * cursor -= 1 ; break ; } } } Ok (signature) }
    };
}

parse_short_keywords!()