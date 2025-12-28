macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! SerializeError {
    () => {
        deps!();
        # [doc = " An error that occurs when serializing an object from this crate."] # [doc = ""] # [doc = " Serialization, as used in this crate, universally refers to the process"] # [doc = " of transforming a structure (like a DFA) into a custom binary format"] # [doc = " represented by `&[u8]`. To this end, serialization is generally infallible."] # [doc = " However, it can fail when caller provided buffer sizes are too small. When"] # [doc = " that occurs, a serialization error is reported."] # [doc = ""] # [doc = " A `SerializeError` provides no introspection capabilities. Its only"] # [doc = " supported operation is conversion to a human readable error message."] # [doc = ""] # [doc = " This error type implements the `std::error::Error` trait only when the"] # [doc = " `std` feature is enabled. Otherwise, this type is defined in all"] # [doc = " configurations."] # [derive (Debug)] pub struct SerializeError { # [doc = " The name of the thing that a buffer is too small for."] # [doc = ""] # [doc = " Currently, the only kind of serialization error is one that is"] # [doc = " committed by a caller: providing a destination buffer that is too"] # [doc = " small to fit the serialized object. This makes sense conceptually,"] # [doc = " since every valid inhabitant of a type should be serializable."] # [doc = ""] # [doc = " This is somewhat exposed in the public API of this crate. For example,"] # [doc = " the `to_bytes_{big,little}_endian` APIs return a `Vec<u8>` and are"] # [doc = " guaranteed to never panic or error. This is only possible because the"] # [doc = " implementation guarantees that it will allocate a `Vec<u8>` that is"] # [doc = " big enough."] # [doc = ""] # [doc = " In summary, if a new serialization error kind needs to be added, then"] # [doc = " it will need careful consideration."] what : & 'static str , }
    };
}

SerializeError!()