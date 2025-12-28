macro_rules! ParamHint {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug)] pub enum ParamHint { None , ArrayFixed (usize) , ArrayRelativeLen (usize) , ArrayRelativeByteLen (usize) , ArrayRelativePtr (usize) , IntoParam , Optional , ValueType , Blittable , Bool , }
    };
}

ParamHint!()