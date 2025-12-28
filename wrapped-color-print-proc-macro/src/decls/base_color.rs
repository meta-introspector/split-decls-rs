macro_rules! deps {
    () => {
        Case!();
        Parser!();
        Error!();
        BaseColor!();
    };
}

macro_rules! base_color {
    () => {
        deps!();
        # [doc = " Parses a base color name, like \"blue\", \"red\", in the given letter case."] fn base_color < 'a > (letter_case : Case) -> impl Parser < 'a , BaseColor > { move | input | { let (input , word) = match letter_case { Case :: Uppercase => { let (input , word) = uppercase_word (input) ? ; (input , Cow :: Owned (word . to_ascii_lowercase ())) } Case :: Lowercase => { let (input , word) = lowercase_word (input) ? ; (input , Cow :: Borrowed (word)) } } ; let base_color = match word . as_ref () { "k" | "black" => BaseColor :: Black , "r" | "red" => BaseColor :: Red , "g" | "green" => BaseColor :: Green , "y" | "yellow" => BaseColor :: Yellow , "b" | "blue" => BaseColor :: Blue , "m" | "magenta" => BaseColor :: Magenta , "c" | "cyan" => BaseColor :: Cyan , "w" | "white" => BaseColor :: White , _ => { return Err (Err :: Error (Error :: new (input , ErrorKind :: Tag , None))) } } ; Ok ((input , base_color)) } }
    };
}

base_color!();