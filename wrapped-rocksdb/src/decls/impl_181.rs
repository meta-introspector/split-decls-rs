macro_rules! deps {
    () => {
        IngestExternalFileOptions!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        unsafe impl Sync for IngestExternalFileOptions { }
    };
}

impl_181!()