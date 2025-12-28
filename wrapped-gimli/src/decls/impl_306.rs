macro_rules! deps {
    () => {
        SubRange!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        unsafe impl < T > Sync for SubRange < T > where T : CloneStableDeref < Target = [u8] > + Debug + Sync { }
    };
}

impl_306!()