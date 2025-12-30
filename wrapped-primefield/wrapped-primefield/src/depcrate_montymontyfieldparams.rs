// Generated macro for MontyFieldParams (trait)
macro_rules! Depcrate_montyMontyFieldParams {
() => {
// Module: crate::monty
// Provides: {"MontyFieldParams"}
// Dependencies: {}
# [doc = " Extension trait for defining additional field parameters beyond the ones provided by"] # [doc = " [`ConstMontyParams`]."] pub trait MontyFieldParams < const LIMBS : usize > : ConstMontyParams < LIMBS > { # [doc = " Size of a field element when serialized as bytes."] type ByteSize : ArraySize ; # [doc = " Byte order to use when serializing a field element as byte."] const BYTE_ORDER : ByteOrder ; # [doc = " Field modulus as a hexadecimal string."] const MODULUS_HEX : & 'static str ; # [doc = " A fixed multiplicative generator of `modulus - 1` order."] # [doc = ""] # [doc = " This element must also be a quadratic nonresidue."] const MULTIPLICATIVE_GENERATOR : u64 ; # [doc = " `T = (modulus - 1) >> S`, where `S = (modulus - 1).trailing_zeros()`"] const T : Uint < LIMBS > ; # [doc = " Optional precomputed `ROOT_OF_UNITY`, otherwise will be computed at compile-time."] const ROOT_OF_UNITY : Option < Uint < LIMBS > > ; }
};
}
