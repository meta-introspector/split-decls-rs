// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl SerializableState for AsconCore { type SerializedStateSize = U40 ; fn serialize (& self) -> SerializedState < Self > { self . state . state . as_bytes () . into () } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let state = ascon :: State :: from (& serialized_state . 0) ; Ok (Self { state : HashCore { state , phantom : PhantomData , } , }) } }
};
}
