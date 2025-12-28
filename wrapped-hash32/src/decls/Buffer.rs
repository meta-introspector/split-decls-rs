macro_rules! Buffer {
    () => {
        # [derive (Debug , Clone , Copy)] # [repr (align (4))] struct Buffer { bytes : MaybeUninit < [u8 ; 4] > , }
    };
}

Buffer!();