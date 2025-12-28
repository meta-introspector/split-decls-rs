macro_rules! deps {
    () => {
        BuiltinInt!();
        BuiltinUint!();
        BuiltinFloat!();
    };
}

macro_rules! BuiltinType {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum BuiltinType { Char , Bool , Str , Int (BuiltinInt) , Uint (BuiltinUint) , Float (BuiltinFloat) , }
    };
}

BuiltinType!();