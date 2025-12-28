macro_rules! BuiltinAttr {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct BuiltinAttr { idx : u32 , }
    };
}

BuiltinAttr!()