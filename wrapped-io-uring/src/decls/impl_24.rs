macro_rules! deps {
    () => {
        Entry!();
        EntryMarker!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl EntryMarker for Entry { const BUILD_FLAGS : u32 = 0 ; }
    };
}

impl_24!();