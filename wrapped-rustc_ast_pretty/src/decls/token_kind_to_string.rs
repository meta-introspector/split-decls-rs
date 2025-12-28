macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! token_kind_to_string {
    () => {
        deps!();
        # [doc = " Print the token kind precisely, without converting `$crate` into its respective crate name."] pub fn token_kind_to_string (tok : & TokenKind) -> Cow < 'static , str > { State :: new () . token_kind_to_string (tok) }
    };
}

token_kind_to_string!();