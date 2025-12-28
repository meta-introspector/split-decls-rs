macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Action { # [doc = " Return ourselves as string representation, similar to what would be passed as argument to a credential helper."] pub fn as_str (& self) -> & 'static str { match self { Action :: Get => "get" , Action :: Store => "store" , Action :: Erase => "erase" , } } }
    };
}

impl_29!();