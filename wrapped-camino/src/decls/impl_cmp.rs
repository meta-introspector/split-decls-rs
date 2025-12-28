macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_cmp {
    () => {
        deps!();
        macro_rules ! impl_cmp { ($ lhs : ty , $ rhs : ty) => { # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < Utf8Path as PartialEq >:: eq (self , other) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < Utf8Path as PartialEq >:: eq (self , other) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < Ordering > { < Utf8Path as PartialOrd >:: partial_cmp (self , other) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < Ordering > { < Utf8Path as PartialOrd >:: partial_cmp (self , other) } } } ; }
    };
}

impl_cmp!()