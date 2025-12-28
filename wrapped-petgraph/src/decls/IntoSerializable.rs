macro_rules! IntoSerializable {
    () => {
        # [doc = " Map to serializeable representation"] pub trait IntoSerializable { type Output ; fn into_serializable (self) -> Self :: Output ; }
    };
}

IntoSerializable!();