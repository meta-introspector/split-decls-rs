// Generated macro for lit_to_mir_constant (function)
macro_rules! Depcrate_constslit_to_mir_constant {
() => {
// Module: crate::consts
// Provides: {"lit_to_mir_constant"}
// Dependencies: {}
# [doc = " Parses a `LitKind` to a `Constant`."] pub fn lit_to_mir_constant (lit : & LitKind , ty : Option < Ty < '_ > >) -> Constant { match * lit { LitKind :: Str (ref is , _) => Constant :: Str (is . to_string ()) , LitKind :: Byte (b) => Constant :: Int (u128 :: from (b)) , LitKind :: ByteStr (ref s , _) | LitKind :: CStr (ref s , _) => Constant :: Binary (s . as_byte_str () . to_vec ()) , LitKind :: Char (c) => Constant :: Char (c) , LitKind :: Int (n , _) => Constant :: Int (n . get ()) , LitKind :: Float (ref is , LitFloatType :: Suffixed (fty)) => match fty { FloatTy :: F16 => Constant :: parse_f16 (is . as_str ()) , FloatTy :: F32 => Constant :: F32 (is . as_str () . parse () . unwrap ()) , FloatTy :: F64 => Constant :: F64 (is . as_str () . parse () . unwrap ()) , FloatTy :: F128 => Constant :: parse_f128 (is . as_str ()) , } , LitKind :: Float (ref is , LitFloatType :: Unsuffixed) => match ty . expect ("type of float is known") . kind () { ty :: Float (FloatTy :: F16) => Constant :: parse_f16 (is . as_str ()) , ty :: Float (FloatTy :: F32) => Constant :: F32 (is . as_str () . parse () . unwrap ()) , ty :: Float (FloatTy :: F64) => Constant :: F64 (is . as_str () . parse () . unwrap ()) , ty :: Float (FloatTy :: F128) => Constant :: parse_f128 (is . as_str ()) , _ => bug ! () , } , LitKind :: Bool (b) => Constant :: Bool (b) , LitKind :: Err (_) => Constant :: Err , } }
};
}
