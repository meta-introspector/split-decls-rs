macro_rules! deps {
    () => {
        CharSpecifier!();
    };
}

macro_rules! parse_char_specifiers {
    () => {
        deps!();
        fn parse_char_specifiers (s : & [char]) -> Vec < CharSpecifier > { let mut cs = Vec :: new () ; let mut i = 0 ; while i < s . len () { if i + 3 <= s . len () && s [i + 1] == '-' { cs . push (CharRange (s [i] , s [i + 2])) ; i += 3 ; } else { cs . push (SingleChar (s [i])) ; i += 1 ; } } cs }
    };
}

parse_char_specifiers!()