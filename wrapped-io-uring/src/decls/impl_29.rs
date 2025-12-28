macro_rules! deps {
    () => {
        EntryMarker!();
        Entry32!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl EntryMarker for Entry32 { const BUILD_FLAGS : u32 = sys :: IORING_SETUP_CQE32 ; }
    };
}

impl_29!()