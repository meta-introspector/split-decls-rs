macro_rules! deps {
    () => {
        Reject!();
    };
}

macro_rules! backslash_x_char {
    () => {
        deps!();
        fn backslash_x_char < I > (chars : & mut I) -> Result < () , Reject > where I : Iterator < Item = (usize , char) > , { next_ch ! (chars @ '0' ..='7') ; next_ch ! (chars @ '0' ..='9' | 'a' ..='f' | 'A' ..='F') ; Ok (()) }
    };
}

backslash_x_char!();