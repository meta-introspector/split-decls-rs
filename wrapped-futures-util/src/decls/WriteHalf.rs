macro_rules! deps {
    () => {
        BiLock!();
    };
}

macro_rules! WriteHalf {
    () => {
        deps!();
        # [doc = " The writable half of an object returned from `AsyncRead::split`."] # [derive (Debug)] pub struct WriteHalf < T > { handle : BiLock < T > , }
    };
}

WriteHalf!()