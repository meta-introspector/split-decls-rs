macro_rules! deps {
    () => {
        Printer!();
        Result!();
        Formatter!();
        Ast!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [doc = " Print a display representation of this Ast."] # [doc = ""] # [doc = " This does not preserve any of the original whitespace formatting that may"] # [doc = " have originally been present in the concrete syntax from which this Ast"] # [doc = " was generated."] # [doc = ""] # [doc = " This implementation uses constant stack space and heap space proportional"] # [doc = " to the size of the `Ast`."] impl core :: fmt :: Display for Ast { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { use crate :: ast :: print :: Printer ; Printer :: new () . print (self , f) } }
    };
}

impl_66!()