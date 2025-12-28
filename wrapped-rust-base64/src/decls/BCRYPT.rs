macro_rules! deps {
    () => {
        Alphabet!();
    };
}

macro_rules! BCRYPT {
    () => {
        deps!();
        # [doc = " The bcrypt alphabet."] pub const BCRYPT : Alphabet = Alphabet :: from_str_unchecked ("./ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789" ,) ;
    };
}

BCRYPT!()