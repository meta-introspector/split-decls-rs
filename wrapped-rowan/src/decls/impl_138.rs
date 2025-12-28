macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < T : ? Sized > Deref for Arc < T > { type Target = T ; # [inline] fn deref (& self) -> & T { & self . inner () . data } }
    };
}

impl_138!();