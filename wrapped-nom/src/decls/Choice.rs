macro_rules! Choice {
    () => {
        # [doc = " Wrapping structure for the [alt()] combinator implementation"] pub struct Choice < T > { parser : T , }
    };
}

Choice!()