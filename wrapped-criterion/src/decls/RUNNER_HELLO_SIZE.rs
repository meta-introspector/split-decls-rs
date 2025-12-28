macro_rules! RUNNER_HELLO_SIZE {
    () => {
        const RUNNER_HELLO_SIZE : usize = RUNNER_MAGIC_NUMBER . len () + (size_of :: < u8 > () * 3) ;
    };
}

RUNNER_HELLO_SIZE!()