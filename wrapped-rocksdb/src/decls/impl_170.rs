macro_rules! deps {
    () => {
        IngestExternalFileOptions!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        unsafe impl Send for IngestExternalFileOptions { }
    };
}

impl_170!()