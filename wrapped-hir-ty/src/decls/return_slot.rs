macro_rules! deps {
    () => {
        LocalId!();
    };
}

macro_rules! return_slot {
    () => {
        deps!();
        fn return_slot < 'db > () -> LocalId < 'db > { LocalId :: from_raw (RawIdx :: from (0)) }
    };
}

return_slot!();