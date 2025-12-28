macro_rules! impl_partial_eq_cow {
    () => {
        # [cfg (feature = "alloc")] macro_rules ! impl_partial_eq_cow { ($ lhs : ty , $ rhs : ty) => { impl <'a > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { let other : & [u8] = (&** other) . as_ref () ; PartialEq :: eq (self . as_bytes () , other) } } impl <'a > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { let this : & [u8] = (&** self) . as_ref () ; PartialEq :: eq (this , other . as_bytes ()) } } } ; }
    };
}

impl_partial_eq_cow!();