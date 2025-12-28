macro_rules! Fmt {
    () => {
        # [derive (Clone)] pub struct Fmt < 'a > { pub original : & 'a Attribute , pub path : ExprPath , }
    };
}

Fmt!();