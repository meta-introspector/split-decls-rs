macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! LcStr {
    () => {
        deps!();
        # [doc = " A variable length string in a load command."] # [doc = ""] # [doc = " The strings are stored just after the load command structure and"] # [doc = " the offset is from the start of the load command structure.  The size"] # [doc = " of the string is reflected in the `cmdsize` field of the load command."] # [doc = " Once again any padded bytes to bring the `cmdsize` field to a multiple"] # [doc = " of 4 bytes must be zero."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct LcStr < E : Endian > { # [doc = " offset to the string"] pub offset : U32 < E > , }
    };
}

LcStr!()