macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! ALPHABETS {
    () => {
        deps!();
        const ALPHABETS : & [alphabet :: Alphabet] = & [alphabet :: URL_SAFE , alphabet :: STANDARD , alphabet :: CRYPT , alphabet :: BCRYPT , alphabet :: IMAP_MUTF7 , alphabet :: BIN_HEX ,] ;
    };
}

ALPHABETS!()