macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! HeapVec {
    () => {
        deps!();
        # [doc = " Simple heap vector implementation."] # [derive (Clone)] pub struct HeapVec { # [doc = " The heap-allocated buffer for the elements."] data : Vec < bigint :: Limb > , }
    };
}

HeapVec!()