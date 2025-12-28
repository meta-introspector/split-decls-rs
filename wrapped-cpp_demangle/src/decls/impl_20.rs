macro_rules! deps {
    () => {
        Result!();
        AutoParseRecursion!();
        ParseContext!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'a > AutoParseRecursion < 'a > { # [inline] fn new (ctx : & 'a ParseContext) -> error :: Result < AutoParseRecursion < 'a > > { ctx . enter_recursion () ? ; Ok (AutoParseRecursion (ctx)) } }
    };
}

impl_20!()