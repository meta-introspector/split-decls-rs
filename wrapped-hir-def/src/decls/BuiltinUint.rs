macro_rules! BuiltinUint {
    () => {
        # [doc = " Different unsigned int types."] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum BuiltinUint { Usize , U8 , U16 , U32 , U64 , U128 , }
    };
}

BuiltinUint!();