macro_rules! deps {
    () => {
        DeserializeStateError!();
        SerializableState!();
        SerializedState!();
    };
}

macro_rules! impl_serializable_state_u8_array {
    () => {
        deps!();
        macro_rules ! impl_serializable_state_u8_array { ($ ($ n : ty) ,*) => { $ (impl SerializableState for [u8 ; <$ n >:: USIZE] { type SerializedStateSize = $ n ; fn serialize (& self) -> SerializedState < Self > { (* self) . into () } fn deserialize (serialized_state : & SerializedState < Self >,) -> Result < Self , DeserializeStateError > { Ok ((* serialized_state) . into ()) } }) * } ; }
    };
}

impl_serializable_state_u8_array!()