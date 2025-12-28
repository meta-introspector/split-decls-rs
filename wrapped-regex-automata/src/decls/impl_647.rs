macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_647 {
    () => {
        deps!();
        impl From < usize > for Ref < 'static > { fn from (x : usize) -> Ref < 'static > { Ref :: Number (x) } }
    };
}

impl_647!();