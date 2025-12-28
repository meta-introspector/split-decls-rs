macro_rules! macro_74 {
    () => {
        thread_local ! { static TEST_ID : RefCell < Option < usize >> = const { RefCell :: new (None) } ; }
    };
}

macro_74!();