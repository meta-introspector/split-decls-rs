macro_rules! deps {
    () => {
        Input!();
        Result!();
        Error!();
    };
}

macro_rules! uppercase_word {
    () => {
        deps!();
        # [doc = " Parses an uppercase word."] pub fn uppercase_word (input : Input < '_ >) -> Result < '_ , & str > { let (input , word) = alpha1 (input) ? ; if word . chars () . all (| c | c . is_ascii_uppercase ()) { Ok ((input , word)) } else { Err (Err :: Error (Error :: new (input , ErrorKind :: Alpha , None))) } }
    };
}

uppercase_word!();