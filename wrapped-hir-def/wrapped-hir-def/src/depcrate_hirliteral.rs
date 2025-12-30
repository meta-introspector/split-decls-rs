// Generated macro for Literal (enum)
macro_rules! Depcrate_hirLiteral {
() => {
// Module: crate::hir
// Provides: {"Literal"}
// Dependencies: {}
# [derive (Debug , Clone , Eq , PartialEq)] pub enum Literal { String (Symbol) , ByteString (Box < [u8] >) , CString (Box < [u8] >) , Char (char) , Bool (bool) , Int (i128 , Option < BuiltinInt >) , Uint (u128 , Option < BuiltinUint >) , Float (FloatTypeWrapper , Option < BuiltinFloat >) , }
};
}
