macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        # [cfg (feature = "borsh")] # [doc = " Requires crate feature `\"borsh\"`"] impl < T , const CAP : usize > borsh :: BorshSerialize for ArrayVec < T , CAP > where T : borsh :: BorshSerialize , { fn serialize < W : borsh :: io :: Write > (& self , writer : & mut W) -> borsh :: io :: Result < () > { < [T] as borsh :: BorshSerialize > :: serialize (self . as_slice () , writer) } }
    };
}

impl_88!()