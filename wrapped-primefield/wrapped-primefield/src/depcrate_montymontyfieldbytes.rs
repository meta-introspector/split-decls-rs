// Generated macro for MontyFieldBytes (type)
macro_rules! Depcrate_montyMontyFieldBytes {
() => {
// Module: crate::monty
// Provides: {"MontyFieldBytes"}
// Dependencies: {}
# [doc = " Serialized representation of a field element."] pub type MontyFieldBytes < MOD , const LIMBS : usize > = Array < u8 , < MOD as MontyFieldParams < LIMBS > > :: ByteSize > ;
};
}
