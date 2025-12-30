// Generated macro for impl_50 (impl)
macro_rules! Depcrate_montyimpl_50 {
() => {
// Module: crate::monty
// Provides: {"impl_50"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Reduce < MontyFieldBytes < MOD , LIMBS > > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , Uint < LIMBS > : ArrayEncoding < ByteSize = MOD :: ByteSize > , { # [inline] fn reduce (bytes : & MontyFieldBytes < MOD , LIMBS >) -> Self { let uint = match MOD :: BYTE_ORDER { ByteOrder :: BigEndian => Uint :: < LIMBS > :: from_be_byte_array (bytes . clone ()) , ByteOrder :: LittleEndian => Uint :: < LIMBS > :: from_le_byte_array (bytes . clone ()) , } ; Self :: reduce (& uint) } }
};
}
