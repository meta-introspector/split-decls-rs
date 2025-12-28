macro_rules! deps {
    () => {
        IntersperseElement!();
        IntersperseElementSimple!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < Item : Clone > IntersperseElement < Item > for IntersperseElementSimple < Item > { fn generate (& mut self) -> Item { self . 0 . clone () } }
    };
}

impl_298!();