macro_rules! deps {
    () => {
        Score!();
        IndentLevel!();
        Token!();
        Indents!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Score { fn for_range (range : Range < u32 > , tokens : & [Token] , indent_of_token : impl Fn (Token) -> IndentLevel ,) -> Score { Indents :: at_token (tokens , range . start as usize , & indent_of_token) . score () + Indents :: at_token (tokens , range . end as usize , & indent_of_token) . score () } }
    };
}

impl_58!();