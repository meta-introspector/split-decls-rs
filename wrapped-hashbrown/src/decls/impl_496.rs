macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl < 'a , T > Clone for Iter < 'a , T > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Iter < 'a , T > { Iter { inner : self . inner . clone () , marker : PhantomData , } } }
    };
}

impl_496!();