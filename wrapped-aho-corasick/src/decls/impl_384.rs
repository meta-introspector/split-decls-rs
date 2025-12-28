macro_rules! deps {
    () => {
        RareByteOffsets!();
        RareByteOffset!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl RareByteOffsets { # [doc = " Create a new empty set of rare byte offsets."] pub (crate) fn empty () -> RareByteOffsets { RareByteOffsets { set : [RareByteOffset :: default () ; 256] } } # [doc = " Add the given offset for the given byte to this set. If the offset is"] # [doc = " greater than the existing offset, then it overwrites the previous"] # [doc = " value and returns false. If there is no previous value set, then this"] # [doc = " sets it and returns true."] pub (crate) fn set (& mut self , byte : u8 , off : RareByteOffset) { self . set [byte as usize] . max = cmp :: max (self . set [byte as usize] . max , off . max) ; } }
    };
}

impl_384!();