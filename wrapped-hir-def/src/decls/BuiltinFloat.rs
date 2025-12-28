macro_rules! BuiltinFloat {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum BuiltinFloat { F16 , F32 , F64 , F128 , }
    };
}

BuiltinFloat!()