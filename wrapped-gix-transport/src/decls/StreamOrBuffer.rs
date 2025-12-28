macro_rules! StreamOrBuffer {
    () => {
        enum StreamOrBuffer { Stream (pipe :: Reader) , Buffer (std :: io :: Cursor < Vec < u8 > >) , }
    };
}

StreamOrBuffer!()