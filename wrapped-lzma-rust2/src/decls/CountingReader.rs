macro_rules! CountingReader {
    () => {
        struct CountingReader < R > { inner : R , bytes_read : u64 , }
    };
}

CountingReader!()