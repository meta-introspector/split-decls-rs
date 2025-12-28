macro_rules! LocalBuf {
    () => {
        pub struct LocalBuf { pub b : [u8 ; OUT_BUF_SIZE] , }
    };
}

LocalBuf!()