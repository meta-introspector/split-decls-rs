macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_cmp_str {
    () => {
        deps!();
        macro_rules ! impl_cmp_str { ($ lhs : ty , $ rhs : ty) => { # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < Utf8Path as PartialEq >:: eq (self , Utf8Path :: new (other)) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < Utf8Path as PartialEq >:: eq (Utf8Path :: new (self) , other) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < std :: cmp :: Ordering > { < Utf8Path as PartialOrd >:: partial_cmp (self , Utf8Path :: new (other)) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < std :: cmp :: Ordering > { < Utf8Path as PartialOrd >:: partial_cmp (Utf8Path :: new (self) , other) } } } ; }
    };
}

impl_cmp_str!();