// Generated macro for is_isize_or_usize (function)
macro_rules! Depcrate_tyis_isize_or_usize {
() => {
// Module: crate::ty
// Provides: {"is_isize_or_usize"}
// Dependencies: {}
# [doc = " Return `true` if the passed `typ` is `isize` or `usize`."] pub fn is_isize_or_usize (typ : Ty < '_ >) -> bool { matches ! (typ . kind () , ty :: Int (IntTy :: Isize) | ty :: Uint (UintTy :: Usize)) }
};
}
