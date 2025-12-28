macro_rules! deps {
    () => {
        State!();
        Token!();
    };
}

macro_rules! token_to_string {
    () => {
        deps!();
        # [doc = " Print the token precisely, without converting `$crate` into its respective crate name."] pub fn token_to_string (token : & Token) -> Cow < 'static , str > { State :: new () . token_to_string (token) }
    };
}

token_to_string!();