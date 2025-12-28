macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for Ref < 'a > { fn from (x : & 'a str) -> Ref < 'a > { Ref :: Named (x) } }
    };
}

impl_646!();