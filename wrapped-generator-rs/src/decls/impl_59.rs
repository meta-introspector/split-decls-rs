macro_rules! deps {
    () => {
        StackBox!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < T > std :: ops :: DerefMut for StackBox < T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . ptr . as_mut () } } }
    };
}

impl_59!()