macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl BitAnd for & FixedBitSet { type Output = FixedBitSet ; fn bitand (self , other : & FixedBitSet) -> FixedBitSet { let (short , long) = { if self . len () <= other . len () { (self . as_simd_slice () , other . as_simd_slice ()) } else { (other . as_simd_slice () , self . as_simd_slice ()) } } ; let mut data = Vec :: from (short) ; for (data , block) in data . iter_mut () . zip (long . iter ()) { * data &= * block ; } let len = core :: cmp :: min (self . len () , other . len ()) ; FixedBitSet :: from_blocks_and_len (data , len) } }
    };
}

impl_129!();