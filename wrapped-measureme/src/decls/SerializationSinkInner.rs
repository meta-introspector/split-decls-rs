macro_rules! SerializationSinkInner {
    () => {
        # [derive (Debug)] struct SerializationSinkInner { buffer : Vec < u8 > , addr : u64 , }
    };
}

SerializationSinkInner!();