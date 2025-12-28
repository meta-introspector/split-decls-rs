macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Event { pub (crate) fn tombstone () -> Self { Event :: Start { kind : TOMBSTONE , forward_parent : None } } }
    };
}

impl_3!();