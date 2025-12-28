macro_rules! deps {
    () => {
        SerializableState!();
    };
}

macro_rules! SubSerializedStateSize {
    () => {
        deps!();
        # [doc = " Alias for `SubSerializedStateSize<T, S> = Diff<T, S::SerializedStateSize>`"] pub type SubSerializedStateSize < T , S > = Diff < T , < S as SerializableState > :: SerializedStateSize > ;
    };
}

SubSerializedStateSize!()