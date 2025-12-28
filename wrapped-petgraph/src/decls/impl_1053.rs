macro_rules! deps {
    () => {
        MappedSequenceVisitor!();
    };
}

macro_rules! impl_1053 {
    () => {
        deps!();
        impl < 'de , F , T , R > MappedSequenceVisitor < T , R , F > where T : Deserialize < 'de > , F : Fn (T) -> Result < R , & 'static str > , { pub fn new (f : F) -> Self { MappedSequenceVisitor { f , marker : PhantomData , } } }
    };
}

impl_1053!()