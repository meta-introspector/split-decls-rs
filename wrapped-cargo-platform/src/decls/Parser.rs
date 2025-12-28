macro_rules! deps {
    () => {
        Tokenizer!();
    };
}

macro_rules! Parser {
    () => {
        deps!();
        struct Parser < 'a > { t : Tokenizer < 'a > , }
    };
}

Parser!();