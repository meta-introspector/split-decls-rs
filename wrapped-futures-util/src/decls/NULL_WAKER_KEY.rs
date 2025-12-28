macro_rules! NULL_WAKER_KEY {
    () => {
        const NULL_WAKER_KEY : usize = usize :: MAX ;
    };
}

NULL_WAKER_KEY!();