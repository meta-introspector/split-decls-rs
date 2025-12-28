macro_rules! deps {
    () => {
        Options!();
        Event!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Options < '_ > { pub (crate) fn to_event_filter (self) -> Option < fn (& Event < '_ >) -> bool > { if self . lossy { Some (discard_nonessential_events) } else { None } } }
    };
}

impl_20!();