macro_rules! deps {
    () => {
        RareByteOffset!();
    };
}

macro_rules! RareByteOffsets {
    () => {
        deps!();
        # [doc = " A set of byte offsets, keyed by byte."] # [derive (Clone , Copy)] struct RareByteOffsets { # [doc = " Each entry corresponds to the maximum offset of the corresponding"] # [doc = " byte across all patterns seen."] set : [RareByteOffset ; 256] , }
    };
}

RareByteOffsets!()