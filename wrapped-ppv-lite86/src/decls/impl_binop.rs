macro_rules! impl_binop {
    () => {
        macro_rules ! impl_binop { ($ vec : ident , $ trait : ident , $ fn : ident , $ impl_fn : ident) => { impl < S3 , S4 , NI > $ trait for $ vec < S3 , S4 , NI > { type Output = Self ; # [inline (always)] fn $ fn (self , rhs : Self) -> Self :: Output { Self :: new (unsafe { $ impl_fn (self . x , rhs . x) }) } } } ; }
    };
}

impl_binop!();