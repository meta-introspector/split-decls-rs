macro_rules! deps {
    () => {
        IndexRange!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : Copy > IndexRange < T > for RangeFrom < T > { # [inline] fn start (& self) -> Option < T > { Some (self . start) } }
    };
}

impl_20!()