macro_rules! ValueRef {
    () => {
        # [doc = " A reference container to encapsulate a tightly packed and typically unallocated byte value that isn't necessarily UTF8 encoded."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct ValueRef < 'a > (# [cfg_attr (feature = "serde" , serde (borrow))] & 'a [u8]) ;
    };
}

ValueRef!();