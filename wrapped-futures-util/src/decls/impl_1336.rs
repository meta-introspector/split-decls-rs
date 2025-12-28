macro_rules! deps {
    () => {
        AbortHandle!();
        AbortRegistration!();
    };
}

macro_rules! impl_1336 {
    () => {
        deps!();
        impl AbortRegistration { # [doc = " Create an [`AbortHandle`] from the given [`AbortRegistration`]."] # [doc = ""] # [doc = " The created [`AbortHandle`] is functionally the same as any other"] # [doc = " [`AbortHandle`]s that are associated with the same [`AbortRegistration`],"] # [doc = " such as the one created by [`AbortHandle::new_pair`]."] pub fn handle (& self) -> AbortHandle { AbortHandle { inner : self . inner . clone () } } }
    };
}

impl_1336!()