macro_rules! deps {
    () => {
        BufferWriter!();
    };
}

macro_rules! Writer {
    () => {
        deps!();
        # [doc = " A terminal target with color awareness."] # [derive (Debug)] pub (crate) struct Writer { inner : BufferWriter , }
    };
}

Writer!()