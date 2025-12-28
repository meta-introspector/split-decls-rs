macro_rules! RawRange {
    () => {
        # [doc = " A raw address range from the `.debug_ranges` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub (crate) struct RawRange { # [doc = " The beginning address of the range."] pub begin : u64 , # [doc = " The first address past the end of the range."] pub end : u64 , }
    };
}

RawRange!()