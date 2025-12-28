macro_rules! deps {
    () => {
        Input!();
        Result!();
        Error!();
    };
}

macro_rules! lowercase_word {
    () => {
        deps!();
        # [doc = " Parses a lowercase word."] pub fn lowercase_word (input : Input < '_ >) -> Result < '_ , & str > { let (input , word) = alpha1 (input) ? ; if word . chars () . all (| c | c . is_ascii_lowercase ()) { Ok ((input , word)) } else { Err (Err :: Error (Error :: new (input , ErrorKind :: Alpha , None))) } }
    };
}

lowercase_word!()