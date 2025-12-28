macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! StackVec {
    () => {
        deps!();
        # [doc = " Simple stack vector implementation."] # [derive (Clone)] pub struct StackVec { # [doc = " The raw buffer for the elements."] data : [mem :: MaybeUninit < bigint :: Limb > ; bigint :: BIGINT_LIMBS] , # [doc = " The number of elements in the array (we never need more than u16::MAX)."] length : u16 , }
    };
}

StackVec!()