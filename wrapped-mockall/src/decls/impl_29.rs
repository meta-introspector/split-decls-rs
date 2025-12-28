macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [doc (hidden)] impl Key { pub fn new < T : 'static + ? Sized > () -> Self { Key (any :: TypeId :: of :: < T > ()) } }
    };
}

impl_29!()