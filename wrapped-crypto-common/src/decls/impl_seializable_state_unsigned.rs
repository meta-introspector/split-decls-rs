macro_rules! deps {
    () => {
        DeserializeStateError!();
        SerializedState!();
        SerializableState!();
    };
}

macro_rules! impl_seializable_state_unsigned {
    () => {
        deps!();
        macro_rules ! impl_seializable_state_unsigned { ($ type : ty , $ type_size : ty) => { impl SerializableState for $ type { type SerializedStateSize = $ type_size ; fn serialize (& self) -> SerializedState < Self > { self . to_le_bytes () . into () } fn deserialize (serialized_state : & SerializedState < Self >,) -> Result < Self , DeserializeStateError > { Ok (<$ type >:: from_le_bytes ((* serialized_state) . into ())) } } } ; }
    };
}

impl_seializable_state_unsigned!();