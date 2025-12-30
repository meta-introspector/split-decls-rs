// Generated macro for impl_105 (impl)
macro_rules! Depcrate_xof_fixedimpl_105 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_105"}
// Dependencies: {}
impl < T : ExtendableOutput + SerializableState , S : ArraySize > SerializableState for XofFixedWrapper < T , S > { type SerializedStateSize = T :: SerializedStateSize ; fn serialize (& self) -> crypto_common :: hazmat :: SerializedState < Self > { self . hash . serialize () } fn deserialize (serialized_state : & crypto_common :: hazmat :: SerializedState < Self > ,) -> Result < Self , crypto_common :: hazmat :: DeserializeStateError > { T :: deserialize (serialized_state) . map (| hash | Self { hash , size : PhantomData , }) } }
};
}
