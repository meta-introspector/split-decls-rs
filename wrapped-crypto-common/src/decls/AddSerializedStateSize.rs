macro_rules! deps {
    () => {
        SerializableState!();
    };
}

macro_rules! AddSerializedStateSize {
    () => {
        deps!();
        # [doc = " Alias for `AddSerializedStateSize<T, S> = Sum<T, S::SerializedStateSize>`"] pub type AddSerializedStateSize < T , S > = Sum < T , < S as SerializableState > :: SerializedStateSize > ;
    };
}

AddSerializedStateSize!();