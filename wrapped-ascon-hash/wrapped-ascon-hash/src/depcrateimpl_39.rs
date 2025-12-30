// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl SerializableState for AsconXofCore { type SerializedStateSize = U40 ; fn serialize (& self) -> SerializedState < Self > { self . state . state . as_bytes () . into () } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let state = ascon :: State :: from (& serialized_state . 0) ; Ok (Self { state : HashCore { state , phantom : PhantomData , } , }) } }
};
}
