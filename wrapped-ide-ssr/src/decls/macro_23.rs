macro_rules! macro_23 {
    () => {
        thread_local ! { pub static RECORDING_MATCH_FAIL_REASONS : Cell < bool > = const { Cell :: new (false) } ; }
    };
}

macro_23!()