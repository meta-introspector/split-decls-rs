macro_rules! MAX_CAPACITY {
    () => {
        const MAX_CAPACITY : usize = ! (OPEN_MASK) ;
    };
}

MAX_CAPACITY!()