macro_rules! deps {
    () => {
        RawTableInner!();
        Tag!();
        RawIterHashInner!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl RawIterHashInner { # [cfg_attr (feature = "inline-more" , inline)] unsafe fn new (table : & RawTableInner , hash : u64) -> Self { let tag_hash = Tag :: full (hash) ; let probe_seq = table . probe_seq (hash) ; let group = Group :: load (table . ctrl (probe_seq . pos)) ; let bitmask = group . match_tag (tag_hash) . into_iter () ; RawIterHashInner { bucket_mask : table . bucket_mask , ctrl : table . ctrl , tag_hash , probe_seq , group , bitmask , } } }
    };
}

impl_110!();