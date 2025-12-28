macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! is_present {
    () => {
        deps!();
        # [doc = " Creates a parser which makes the parser optional and returns true if the parse was successful."] pub fn is_present < 'a , P , V > (parser : P) -> impl Parser < 'a , bool > where P : Parser < 'a , V > , { map (opt (parser) , | v | v . is_some ()) }
    };
}

is_present!();