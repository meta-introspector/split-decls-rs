macro_rules! deps {
    () => {
        BreakToken!();
        BeginToken!();
    };
}

macro_rules! Token {
    () => {
        deps!();
        # [derive (PartialEq)] pub (crate) enum Token { String (Cow < 'static , str >) , Break (BreakToken) , Begin (BeginToken) , End , }
    };
}

Token!();