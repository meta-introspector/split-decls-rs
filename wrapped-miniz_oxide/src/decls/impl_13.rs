macro_rules! deps {
    () => {
        LocalBuf!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Default for LocalBuf { fn default () -> LocalBuf { LocalBuf { b : [0 ; OUT_BUF_SIZE] , } } }
    };
}

impl_13!();