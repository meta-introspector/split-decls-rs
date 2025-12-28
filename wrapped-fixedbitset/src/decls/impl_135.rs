macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl BitXor for & FixedBitSet { type Output = FixedBitSet ; fn bitxor (self , other : & FixedBitSet) -> FixedBitSet { let (short , long) = { if self . len () <= other . len () { (self . as_simd_slice () , other . as_simd_slice ()) } else { (other . as_simd_slice () , self . as_simd_slice ()) } } ; let mut data = Vec :: from (long) ; for (data , block) in data . iter_mut () . zip (short . iter ()) { * data ^= * block ; } let len = core :: cmp :: max (self . len () , other . len ()) ; FixedBitSet :: from_blocks_and_len (data , len) } }
    };
}

impl_135!();