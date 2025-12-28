macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! IS_WRITE_STOPPED {
    () => {
        deps!();
        # [doc = " \"rocksdb.is-write-stopped\" - Return 1 if write has been stopped."] pub const IS_WRITE_STOPPED : & PropName = property ! ("is-write-stopped") ;
    };
}

IS_WRITE_STOPPED!()