macro_rules! deps {
    () => {
        AutoFinish!();
        LzmaWriter!();
        Write!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < W : Write > AutoFinish for LzmaWriter < W > { fn finish_ignore_error (self) { let _ = self . finish () ; } }
    };
}

impl_227!()