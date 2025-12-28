macro_rules! macro_25 {
    () => {
        thread_local ! (static IN_SCOPE : RefCell < bool > = const { RefCell :: new (false) }) ;
    };
}

macro_25!();