macro_rules! fwd_binop_x4 {
    () => {
        macro_rules ! fwd_binop_x4 { ($ trait : ident , $ fn : ident) => { impl < W : $ trait + Copy > $ trait for x4 < W > { type Output = x4 < W :: Output >; # [inline (always)] fn $ fn (self , rhs : Self) -> Self :: Output { x4 ([self . 0 [0] .$ fn (rhs . 0 [0]) , self . 0 [1] .$ fn (rhs . 0 [1]) , self . 0 [2] .$ fn (rhs . 0 [2]) , self . 0 [3] .$ fn (rhs . 0 [3]) ,]) } } } ; }
    };
}

fwd_binop_x4!();