// Generated macro for impl_28 (impl)
macro_rules! Depcrate_lms_modesimpl_28 {
() => {
// Module: crate::lms::modes
// Provides: {"impl_28"}
// Dependencies: {}
impl < OtsMode : LmsOtsMode , Hasher : Digest , HLen : ArraySize , const M : usize , const H : usize , const TC : u32 , > LmsMode for LmsModeInternal < OtsMode , Hasher , HLen , M , H , TC > where HLen : Add < typenum :: B1 > , U1 : Shl < < HLen as Add < B1 > > :: Output > , Shleft < U1 , < HLen as Add < B1 > > :: Output > : Sub < B1 > , Sub1 < Shleft < U1 , < HLen as Add < B1 > > :: Output > > : ArraySize , { type OtsMode = OtsMode ; type Hasher = Hasher ; type TreeLen = Sub1 < Shleft < U1 , Add1 < HLen > > > ; type HLen = HLen ; const M : usize = M ; const H : usize = H ; const LEAVES : u32 = 1 << H ; const TREE_NODES : u32 = (1 << (H + 1)) - 1 ; }
};
}
