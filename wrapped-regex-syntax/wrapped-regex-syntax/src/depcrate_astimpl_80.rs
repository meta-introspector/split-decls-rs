// Generated macro for impl_80 (impl)
macro_rules! Depcrate_astimpl_80 {
() => {
// Module: crate::ast
// Provides: {"impl_80"}
// Dependencies: {}
# [doc = " Print a display representation of this Ast."] # [doc = ""] # [doc = " This does not preserve any of the original whitespace formatting that may"] # [doc = " have originally been present in the concrete syntax from which this Ast"] # [doc = " was generated."] # [doc = ""] # [doc = " This implementation uses constant stack space and heap space proportional"] # [doc = " to the size of the `Ast`."] impl core :: fmt :: Display for Ast { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { use crate :: ast :: print :: Printer ; Printer :: new () . print (self , f) } }
};
}
