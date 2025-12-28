macro_rules! deps {
    () => {
        EntryMarker!();
        Entry!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl EntryMarker for Entry { const BUILD_FLAGS : u32 = 0 ; }
    };
}

impl_140!();