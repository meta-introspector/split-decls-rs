macro_rules! macro_375 {
    () => {
        thread_local ! { static SEED_RAND : RefCell < Rand64 > = RefCell :: new (Rand64 :: new (SystemTime :: now () . duration_since (UNIX_EPOCH) . expect ("Time went backwards") . as_millis ())) ; }
    };
}

macro_375!();