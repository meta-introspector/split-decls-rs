// Generated macro for impl_171 (impl)
macro_rules! Depcrateimpl_171 {
() => {
// Module: crate
// Provides: {"impl_171"}
// Dependencies: {}
impl Integer { # [doc = " Returns `true` if the integer can be represented as `i64`."] # [inline] # [must_use] pub const fn is_i64 (& self) -> bool { match self . n { IntPriv :: PosInt (n) => n <= std :: i64 :: MAX as u64 , IntPriv :: NegInt (..) => true , } } # [doc = " Returns `true` if the integer can be represented as `u64`."] # [inline] # [must_use] pub const fn is_u64 (& self) -> bool { match self . n { IntPriv :: PosInt (..) => true , IntPriv :: NegInt (..) => false , } } # [doc = " Returns the integer represented as `i64` if possible, or else `None`."] # [inline] # [must_use] pub fn as_i64 (& self) -> Option < i64 > { match self . n { IntPriv :: PosInt (n) => n . try_into () . ok () , IntPriv :: NegInt (n) => Some (n) , } } # [doc = " Returns the integer represented as `u64` if possible, or else `None`."] # [inline] # [must_use] pub fn as_u64 (& self) -> Option < u64 > { match self . n { IntPriv :: PosInt (n) => Some (n) , IntPriv :: NegInt (n) => n . try_into () . ok () , } } # [doc = " Returns the integer represented as `f64` if possible, or else `None`."] # [inline] # [must_use] pub fn as_f64 (& self) -> Option < f64 > { match self . n { IntPriv :: PosInt (n) => Some (n as _) , IntPriv :: NegInt (n) => Some (n as _) , } } }
};
}
