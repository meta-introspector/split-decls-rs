macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl From < Id > for usize { fn from (src : Id) -> usize { src . id } }
    };
}

impl_167!()