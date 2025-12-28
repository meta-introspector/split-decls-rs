macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < T > DerefMut for Sp < T > { fn deref_mut (& mut self) -> & mut T { & mut self . val } }
    };
}

impl_94!();