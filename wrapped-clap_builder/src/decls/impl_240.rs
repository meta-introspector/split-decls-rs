macro_rules! deps {
    () => {
        Error!();
        StyledStr!();
        Result!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl std :: fmt :: Write for StyledStr { # [inline] fn write_str (& mut self , s : & str) -> Result < () , std :: fmt :: Error > { self . 0 . push_str (s) ; Ok (()) } # [inline] fn write_char (& mut self , c : char) -> Result < () , std :: fmt :: Error > { self . 0 . push (c) ; Ok (()) } }
    };
}

impl_240!();