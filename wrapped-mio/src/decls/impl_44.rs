macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl From < Token > for usize { fn from (val : Token) -> usize { val . 0 } }
    };
}

impl_44!();