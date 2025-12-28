macro_rules! CopyAsciiResult {
    () => {
        pub enum CopyAsciiResult < T , U > { Stop (T) , GoOn (U) , }
    };
}

CopyAsciiResult!()