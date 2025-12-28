macro_rules! BuiltinInt {
    () => {
        # [doc = " Different signed int types."] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum BuiltinInt { Isize , I8 , I16 , I32 , I64 , I128 , }
    };
}

BuiltinInt!()