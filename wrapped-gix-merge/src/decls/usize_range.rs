macro_rules! usize_range {
    () => {
        fn usize_range (range : & Range < u32 >) -> Range < usize > { range . start as usize .. range . end as usize }
    };
}

usize_range!();