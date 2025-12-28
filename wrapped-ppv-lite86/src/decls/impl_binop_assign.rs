macro_rules! impl_binop_assign {
    () => {
        macro_rules ! impl_binop_assign { ($ vec : ident , $ trait : ident , $ fn_assign : ident , $ fn : ident) => { impl < S3 , S4 , NI > $ trait for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: Copy , { # [inline (always)] fn $ fn_assign (& mut self , rhs : Self) { * self = self .$ fn (rhs) ; } } } ; }
    };
}

impl_binop_assign!();