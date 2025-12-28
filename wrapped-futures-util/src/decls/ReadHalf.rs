macro_rules! deps {
    () => {
        BiLock!();
    };
}

macro_rules! ReadHalf {
    () => {
        deps!();
        # [doc = " The readable half of an object returned from `AsyncRead::split`."] # [derive (Debug)] pub struct ReadHalf < T > { handle : BiLock < T > , }
    };
}

ReadHalf!();