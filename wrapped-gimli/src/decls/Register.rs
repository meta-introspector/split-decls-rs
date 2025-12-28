macro_rules! Register {
    () => {
        # [doc = " A DWARF register number."] # [doc = ""] # [doc = " The meaning of this value is ABI dependent. This is generally encoded as"] # [doc = " a ULEB128, but supported architectures need 16 bits at most."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct Register (pub u16) ;
    };
}

Register!()