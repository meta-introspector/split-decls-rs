macro_rules! deps {
    () => {
        SubRange!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        unsafe impl < T > Send for SubRange < T > where T : CloneStableDeref < Target = [u8] > + Debug + Send { }
    };
}

impl_305!()