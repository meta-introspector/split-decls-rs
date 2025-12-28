macro_rules! N64_SIZE {
    () => {
        const N64_SIZE : usize = size_of :: < u64 > () ;
    };
}

N64_SIZE!()