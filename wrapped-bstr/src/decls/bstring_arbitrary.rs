macro_rules! deps {
    () => {
        BString!();
    };
}

macro_rules! bstring_arbitrary {
    () => {
        deps!();
        # [cfg (all (test , feature = "alloc"))] mod bstring_arbitrary { use alloc :: { boxed :: Box , vec :: Vec } ; use crate :: bstring :: BString ; use quickcheck :: { Arbitrary , Gen } ; impl Arbitrary for BString { fn arbitrary (g : & mut Gen) -> BString { BString :: from (Vec :: < u8 > :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = BString > > { Box :: new (self . as_vec () . shrink () . map (BString :: from)) } } }
    };
}

bstring_arbitrary!()