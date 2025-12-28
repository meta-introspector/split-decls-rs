macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! other_23 {
    () => {
        deps!();
        # [doc = " A 64-bit value represented as a pair of 32-bit values."] # [doc = ""] # [doc = " This type is `#[repr(C)]`, both fields have the same in-memory representation"] # [doc = " and are plain old data types, so access to the fields is always safe."] # [allow (dead_code)] # [derive (Clone , Copy)] # [repr (C)] pub (crate) union MaybeUninit64 { pub (crate) whole : MaybeUninit < u64 > , pub (crate) pair : Pair < u32 > , }
    };
}

other_23!();