macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        # [doc = " Consuming"] impl Entry < '_ > { # [doc = " Return the contained object."] pub fn detach (self) -> gix_object :: tree :: Entry { self . inner } }
    };
}

impl_234!();