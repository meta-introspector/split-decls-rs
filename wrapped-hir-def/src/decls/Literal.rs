macro_rules! deps {
    () => {
        BuiltinUint!();
        BuiltinFloat!();
        BuiltinInt!();
        FloatTypeWrapper!();
    };
}

macro_rules! Literal {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub enum Literal { String (Symbol) , ByteString (Box < [u8] >) , CString (Box < [u8] >) , Char (char) , Bool (bool) , Int (i128 , Option < BuiltinInt >) , Uint (u128 , Option < BuiltinUint >) , Float (FloatTypeWrapper , Option < BuiltinFloat >) , }
    };
}

Literal!()