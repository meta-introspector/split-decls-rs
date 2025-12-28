macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! BACKGROUND_ERRORS {
    () => {
        deps!();
        # [doc = " \"rocksdb.background-errors\" - returns accumulated number of background"] # [doc = " errors."] pub const BACKGROUND_ERRORS : & PropName = property ! ("background-errors") ;
    };
}

BACKGROUND_ERRORS!()