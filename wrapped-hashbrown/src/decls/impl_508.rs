macro_rules! deps {
    () => {
        IterHash!();
    };
}

macro_rules! impl_508 {
    () => {
        deps!();
        impl < 'a , T > Clone for IterHash < 'a , T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> IterHash < 'a , T > { IterHash { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_508!()