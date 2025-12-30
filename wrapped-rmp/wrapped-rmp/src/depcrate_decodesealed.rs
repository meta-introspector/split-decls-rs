// Generated macro for sealed (module)
macro_rules! Depcrate_decodesealed {
() => {
// Module: crate::decode
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { pub trait Sealed { } # [cfg (feature = "std")] impl < T : ? Sized + std :: io :: Read > Sealed for T { } # [cfg (not (feature = "std"))] impl < 'a > Sealed for & 'a [u8] { } impl Sealed for super :: Bytes < '_ > { } }
};
}
