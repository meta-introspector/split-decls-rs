macro_rules! deps {
    () => {
        AdhocKind!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < T > AdhocKind for & T where T : ? Sized + Display + Debug + Send + Sync + 'static { }
    };
}

impl_93!()