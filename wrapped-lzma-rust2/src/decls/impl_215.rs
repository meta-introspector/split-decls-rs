macro_rules! deps {
    () => {
        AutoFinish!();
        Write!();
        Lzma2Writer!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < W : Write > AutoFinish for Lzma2Writer < W > { fn finish_ignore_error (self) { let _ = self . finish () ; } }
    };
}

impl_215!()