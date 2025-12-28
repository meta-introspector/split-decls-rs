macro_rules! Rewind {
    () => {
        # [doc = " Combine a buffer with an IO, rewinding reads to use the buffer."] # [derive (Debug)] pub (crate) struct Rewind < T > { pre : Option < Bytes > , inner : T , }
    };
}

Rewind!()