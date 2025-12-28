macro_rules! impl_partial_ord {
    () => {
        macro_rules ! impl_partial_ord { ($ lhs : ty , $ rhs : ty) => { impl <'a > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < Ordering > { let other : & [u8] = other . as_ref () ; PartialOrd :: partial_cmp (self . as_bytes () , other) } } impl <'a > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < Ordering > { let this : & [u8] = self . as_ref () ; PartialOrd :: partial_cmp (this , other . as_bytes ()) } } } ; }
    };
}

impl_partial_ord!()