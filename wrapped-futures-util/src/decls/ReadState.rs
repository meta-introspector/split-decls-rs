macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! ReadState {
    () => {
        deps!();
        # [derive (Debug)] enum ReadState < T : AsRef < [u8] > > { Ready { chunk : T , chunk_start : usize } , PendingChunk , Eof , }
    };
}

ReadState!()