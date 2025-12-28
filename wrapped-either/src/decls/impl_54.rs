macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < L , R , Target > AsMut < Target > for Either < L , R > where L : AsMut < Target > , R : AsMut < Target > , { fn as_mut (& mut self) -> & mut Target { for_both ! (self , inner => inner . as_mut ()) } }
    };
}

impl_54!();