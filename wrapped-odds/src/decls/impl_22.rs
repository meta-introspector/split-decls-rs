macro_rules! deps {
    () => {
        IndexRange!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T : Copy > IndexRange < T > for Range < T > { # [inline] fn start (& self) -> Option < T > { Some (self . start) } # [inline] fn end (& self) -> Option < T > { Some (self . end) } }
    };
}

impl_22!();