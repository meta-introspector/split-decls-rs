// Generated macro for sealed (module)
macro_rules! Depcrate_encodesealed {
() => {
// Module: crate::encode
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { pub trait Sealed { } # [cfg (feature = "std")] impl < T : ? Sized + std :: io :: Write > Sealed for T { } # [cfg (not (feature = "std"))] impl Sealed for & mut [u8] { } # [cfg (not (feature = "std"))] impl Sealed for alloc :: vec :: Vec < u8 > { } impl Sealed for super :: ByteBuf { } }
};
}
