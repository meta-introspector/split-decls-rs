macro_rules! deps {
    () => {
        WritableTarget!();
        WriteStyle!();
    };
}

macro_rules! BufferWriter {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct BufferWriter { target : WritableTarget , write_style : WriteStyle , }
    };
}

BufferWriter!()