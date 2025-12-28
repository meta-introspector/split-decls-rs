macro_rules! UnalignedU16Slice {
    () => {
        # [derive (Debug , Copy , Clone)] struct UnalignedU16Slice { ptr : * const u8 , len : usize , }
    };
}

UnalignedU16Slice!();