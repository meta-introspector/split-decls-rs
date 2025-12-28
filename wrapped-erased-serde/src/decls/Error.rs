macro_rules! deps {
    () => {
        ErrorImpl!();
        Deserializer!();
        Serializer!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Error when a `Serializer` or `Deserializer` trait object fails."] pub struct Error { imp : Box < ErrorImpl > , }
    };
}

Error!();