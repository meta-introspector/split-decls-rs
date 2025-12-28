macro_rules! fwd_unop_x2 {
    () => {
        macro_rules ! fwd_unop_x2 { ($ fn : ident) => { # [inline (always)] fn $ fn (self) -> Self { x2 :: new ([self . 0 [0] .$ fn () , self . 0 [1] .$ fn ()]) } } ; }
    };
}

fwd_unop_x2!();