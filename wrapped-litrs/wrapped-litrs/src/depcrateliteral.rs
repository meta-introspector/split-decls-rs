// Generated macro for Literal (enum)
macro_rules! DepcrateLiteral {
() => {
// Module: crate
// Provides: {"Literal"}
// Dependencies: {}
# [doc = " A literal. This is the main type of this library."] # [doc = ""] # [doc = " This type is generic over the underlying buffer `B`, which can be `&str` or"] # [doc = " `String`."] # [doc = ""] # [doc = " To create this type, you have to either call [`Literal::parse`] with an"] # [doc = " input string or use the `From<_>` impls of this type. The impls are only"] # [doc = " available of the corresponding crate features are enabled (they are enabled"] # [doc = " by default)."] # [derive (Debug , Clone , PartialEq , Eq)] # [non_exhaustive] pub enum Literal < B : Buffer > { Bool (BoolLit) , Integer (IntegerLit < B >) , Float (FloatLit < B >) , Char (CharLit < B >) , String (StringLit < B >) , Byte (ByteLit < B >) , ByteString (ByteStringLit < B >) , CString (CStringLit < B >) , }
};
}
