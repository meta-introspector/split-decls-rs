macro_rules! deps {
    () => {
        MatchOptions!();
        CharSpecifier!();
    };
}

macro_rules! in_char_specifiers {
    () => {
        deps!();
        fn in_char_specifiers (specifiers : & [CharSpecifier] , c : char , options : MatchOptions) -> bool { for & specifier in specifiers . iter () { match specifier { SingleChar (sc) => { if chars_eq (c , sc , options . case_sensitive) { return true ; } } CharRange (start , end) => { if ! options . case_sensitive && c . is_ascii () && start . is_ascii () && end . is_ascii () { let start = start . to_ascii_lowercase () ; let end = end . to_ascii_lowercase () ; let start_up = start . to_uppercase () . next () . unwrap () ; let end_up = end . to_uppercase () . next () . unwrap () ; if start != start_up && end != end_up { let c = c . to_ascii_lowercase () ; if c >= start && c <= end { return true ; } } } if c >= start && c <= end { return true ; } } } } false }
    };
}

in_char_specifiers!();