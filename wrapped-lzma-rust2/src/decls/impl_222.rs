macro_rules! deps {
    () => {
        Write!();
        Lzma2WriterMt!();
        AutoFinish!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl < W : Write > AutoFinish for Lzma2WriterMt < W > { fn finish_ignore_error (self) { let _ = self . finish () ; } }
    };
}

impl_222!()