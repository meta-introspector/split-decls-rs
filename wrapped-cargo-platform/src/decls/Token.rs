macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! Token {
    () => {
        deps!();
        # [derive (PartialEq)] enum Token < 'a > { LeftParen , RightParen , Ident (bool , & 'a str) , Comma , Equals , String (& 'a str) , }
    };
}

Token!()