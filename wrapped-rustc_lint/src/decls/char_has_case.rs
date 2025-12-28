macro_rules! char_has_case {
    () => {
        # [doc = " Some unicode characters *have* case, are considered upper case or lower case, but they *can't*"] # [doc = " be upper cased or lower cased. For the purposes of the lint suggestion, we care about being able"] # [doc = " to change the char's case."] fn char_has_case (c : char) -> bool { let mut l = c . to_lowercase () ; let mut u = c . to_uppercase () ; while let Some (l) = l . next () { match u . next () { Some (u) if l != u => return true , _ => { } } } u . next () . is_some () }
    };
}

char_has_case!();