macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! NativeEndian {
    () => {
        deps!();
        # [cfg (target_endian = "big")] # [allow (non_upper_case_globals)] # [doc (hidden)] pub const NativeEndian : BigEndian = BigEndian ;
    };
}

NativeEndian!();