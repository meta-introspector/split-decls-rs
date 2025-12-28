macro_rules! deps {
    () => {
        Write!();
        AutoFinish!();
        LzmaWriter!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < W : Write > AutoFinish for LzmaWriter < W > { fn finish_ignore_error (self) { let _ = self . finish () ; } }
    };
}

impl_227!();