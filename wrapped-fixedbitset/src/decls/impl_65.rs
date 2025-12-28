macro_rules! deps {
    () => {
        IndexRange!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T : Copy > IndexRange < T > for RangeTo < T > { # [inline] fn end (& self) -> Option < T > { Some (self . end) } }
    };
}

impl_65!()