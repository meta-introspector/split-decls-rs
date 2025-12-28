macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! spaced {
    () => {
        deps!();
        # [doc = " Creates a parser which accpets spaces around the original parsed input."] pub fn spaced < 'a , P , V > (parser : P) -> impl Parser < 'a , V > where P : Parser < 'a , V > , { delimited (multispace0 , parser , multispace0 ,) }
    };
}

spaced!()