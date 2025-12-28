macro_rules! deps {
    () => {
        Incoming!();
    };
}

macro_rules! Tx {
    () => {
        deps!();
        enum Tx { # [cfg (feature = "http1")] Http1 (conn :: http1 :: SendRequest < crate :: body :: Incoming >) , # [cfg (feature = "http2")] Http2 (conn :: http2 :: SendRequest < crate :: body :: Incoming >) , }
    };
}

Tx!()