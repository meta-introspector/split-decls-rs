macro_rules! deps {
    () => {
        IntersperseElement!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < Item , F : FnMut () -> Item > IntersperseElement < Item > for F { fn generate (& mut self) -> Item { self () } }
    };
}

impl_301!()