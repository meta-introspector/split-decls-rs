macro_rules! deps {
    () => {
        Push!();
        Direction!();
        Fetch!();
    };
}

macro_rules! impl_907 {
    () => {
        deps!();
        impl Direction { # [doc = " Return ourselves as string suitable for use as verb in an english sentence."] pub fn as_str (& self) -> & 'static str { match self { Direction :: Push => "push" , Direction :: Fetch => "fetch" , } } }
    };
}

impl_907!();