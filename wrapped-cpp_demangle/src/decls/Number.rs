macro_rules! Number {
    () => {
        # [doc = " The `<number>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <number> ::= [n] <non-negative decimal integer>"] # [doc = " ```"] type Number = isize ;
    };
}

Number!();