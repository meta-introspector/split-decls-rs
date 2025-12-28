macro_rules! N32_SIZE {
    () => {
        const N32_SIZE : usize = size_of :: < u32 > () ;
    };
}

N32_SIZE!()