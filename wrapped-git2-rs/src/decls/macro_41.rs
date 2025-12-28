macro_rules! macro_41 {
    () => {
        thread_local ! (static LAST_ERROR : RefCell < Option < Box < dyn Any + Send >>> = { RefCell :: new (None) }) ;
    };
}

macro_41!();