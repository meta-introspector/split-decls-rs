// Generated macro for impl_157 (impl)
macro_rules! Depcrate_jacobiimpl_157 {
() => {
// Module: crate::jacobi
// Provides: {"impl_157"}
// Dependencies: {}
impl JacobiSymbol { # [doc = " Determine if the symbol is zero."] pub const fn is_zero (& self) -> ConstChoice { ConstChoice :: from_i64_eq (* self as i8 as i64 , 0) } # [doc = " Determine if the symbol is one."] pub const fn is_one (& self) -> ConstChoice { ConstChoice :: from_i64_eq (* self as i8 as i64 , 1) } # [doc = " Determine if the symbol is minus one."] pub const fn is_minus_one (& self) -> ConstChoice { ConstChoice :: from_i64_eq (* self as i8 as i64 , - 1) } # [doc = " Negate the symbol."] pub const fn neg (self) -> Self { match self { Self :: Zero => Self :: Zero , Self :: One => Self :: MinusOne , Self :: MinusOne => Self :: One , } } pub (crate) const fn from_i8 (value : i8) -> Self { match value { 0 => Self :: Zero , 1 => Self :: One , - 1 => Self :: MinusOne , _ => panic ! ("invalid value for Jacobi symbol") , } } }
};
}
