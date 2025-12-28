macro_rules! deps {
    () => {
        EntryMarker!();
        Entry128!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl EntryMarker for Entry128 { const BUILD_FLAGS : u32 = sys :: IORING_SETUP_SQE128 ; }
    };
}

impl_145!()