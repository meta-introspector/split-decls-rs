macro_rules! deps {
    () => {
        SerializableState!();
    };
}

macro_rules! SerializedState {
    () => {
        deps!();
        # [doc = " Serialized internal state."] pub type SerializedState < T > = Array < u8 , < T as SerializableState > :: SerializedStateSize > ;
    };
}

SerializedState!()