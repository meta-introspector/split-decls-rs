macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! ARC_MAX_BYTES {
    () => {
        deps!();
        # [doc = " Maximum number of bytes supported in an arc."] # [doc = ""] # [doc = " Note that OIDs are base 128 encoded (with continuation bits), so we must consider how many bytes"] # [doc = " are required when each byte can only represent 7-bits of the input."] const ARC_MAX_BYTES : usize = (Arc :: BITS as usize) . div_ceil (7) ;
    };
}

ARC_MAX_BYTES!();