macro_rules! deps {
    () => {
        BitMaskIter!();
        Tag!();
        ProbeSeq!();
    };
}

macro_rules! RawIterHashInner {
    () => {
        deps!();
        # [derive (Clone)] struct RawIterHashInner { bucket_mask : usize , ctrl : NonNull < u8 > , tag_hash : Tag , probe_seq : ProbeSeq , group : Group , bitmask : BitMaskIter , }
    };
}

RawIterHashInner!()