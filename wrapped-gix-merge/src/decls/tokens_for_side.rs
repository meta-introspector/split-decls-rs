macro_rules! deps {
    () => {
        Side!();
    };
}

macro_rules! tokens_for_side {
    () => {
        deps!();
        fn tokens_for_side < 'a > (side : Side , input : & 'a InternedInput < & [u8] > , current_tokens : & 'a [Token] ,) -> & 'a [Token] { match side { Side :: Current => current_tokens , Side :: Other => & input . after , Side :: Ancestor => & input . before , } }
    };
}

tokens_for_side!();