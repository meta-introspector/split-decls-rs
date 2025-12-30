// Generated macro for bitflags (module)
macro_rules! Depcrate_rtbitflags {
() => {
// Module: crate::rt
// Provides: {"bitflags"}
// Dependencies: {}
# [cfg (not (feature = "bitflags"))] pub mod bitflags { # [macro_export] macro_rules ! bitflags { ($ (# [$ attr : meta]) * $ vis : vis struct $ name : ident : $ repr : ty { $ ($ (# [$ flag_attr : meta]) * const $ flag : ident = $ val : expr ;) * }) => { $ (# [$ attr]) * $ vis struct $ name { bits : $ repr , } impl $ name { $ ($ (# [$ flag_attr]) * $ vis const $ flag : Self = Self { bits : $ val } ;) * $ vis fn empty () -> Self { Self { bits : 0 } } $ vis fn from_bits_retain (bits : $ repr) -> Self { Self { bits } } $ vis fn bits (& self) -> $ repr { self . bits } } impl core :: ops :: BitOr <$ name > for $ name { type Output = Self ; fn bitor (self , rhs : $ name) -> $ name { Self { bits : self . bits | rhs . bits } } } impl core :: ops :: BitAnd <$ name > for $ name { type Output = Self ; fn bitand (self , rhs : $ name) -> $ name { Self { bits : self . bits & rhs . bits } } } impl core :: ops :: BitXor <$ name > for $ name { type Output = Self ; fn bitxor (self , rhs : $ name) -> $ name { Self { bits : self . bits ^ rhs . bits } } } } ; } pub use crate :: bitflags ; }
};
}
