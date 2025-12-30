// Generated macro for bstring_arbitrary (module)
macro_rules! Depcrate_implsbstring_arbitrary {
() => {
// Module: crate::impls
// Provides: {"bstring_arbitrary"}
// Dependencies: {}
# [cfg (all (test , feature = "alloc"))] mod bstring_arbitrary { use alloc :: { boxed :: Box , vec :: Vec } ; use crate :: bstring :: BString ; use quickcheck :: { Arbitrary , Gen } ; impl Arbitrary for BString { fn arbitrary (g : & mut Gen) -> BString { BString :: from (Vec :: < u8 > :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = BString > > { Box :: new (self . as_vec () . shrink () . map (BString :: from)) } } }
};
}
