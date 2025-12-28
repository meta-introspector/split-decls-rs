macro_rules! macro_3 {
    () => {
        thread_local ! (static IN_SCOPE : RefCell < bool > = const { RefCell :: new (false) }) ;
    };
}

macro_3!()