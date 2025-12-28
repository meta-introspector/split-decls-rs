macro_rules! deps {
    () => {
        ReadRef!();
        Symbol!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < 'data , 'file , R : ReadRef < 'data > > read :: private :: Sealed for Symbol < 'data , 'file , R > { }
    };
}

impl_164!();