macro_rules! macro_152 {
    () => {
        thread_local ! (static LAST_ERROR : RefCell < Option < Box < dyn Any + Send >>> = { RefCell :: new (None) }) ;
    };
}

macro_152!()