macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (feature = "borsh")] # [doc = " Requires crate feature `\"borsh\"`"] impl < const CAP : usize > borsh :: BorshSerialize for ArrayString < CAP > { fn serialize < W : borsh :: io :: Write > (& self , writer : & mut W) -> borsh :: io :: Result < () > { < str as borsh :: BorshSerialize > :: serialize (& * self , writer) } }
    };
}

impl_31!()