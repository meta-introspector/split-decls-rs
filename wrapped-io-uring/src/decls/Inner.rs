macro_rules! deps {
    () => {
        EntryMarker!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        pub (crate) struct Inner < E : EntryMarker > { pub (crate) head : * const atomic :: AtomicU32 , pub (crate) tail : * const atomic :: AtomicU32 , pub (crate) ring_mask : u32 , pub (crate) ring_entries : u32 , pub (crate) flags : * const atomic :: AtomicU32 , dropped : * const atomic :: AtomicU32 , pub (crate) sqes : * mut E , }
    };
}

Inner!();