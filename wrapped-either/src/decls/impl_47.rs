macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < L , R , Target > AsRef < Target > for Either < L , R > where L : AsRef < Target > , R : AsRef < Target > , { fn as_ref (& self) -> & Target { for_both ! (self , inner => inner . as_ref ()) } }
    };
}

impl_47!()