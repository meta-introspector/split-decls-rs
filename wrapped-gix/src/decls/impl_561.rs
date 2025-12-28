macro_rules! deps {
    () => {
        Snapshot!();
    };
}

macro_rules! impl_561 {
    () => {
        deps!();
        impl Deref for Snapshot < '_ > { type Target = gix_config :: File < 'static > ; fn deref (& self) -> & Self :: Target { self . plumbing () } }
    };
}

impl_561!();