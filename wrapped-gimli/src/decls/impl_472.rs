macro_rules! deps {
    () => {
        DebugLookup!();
        Reader!();
        LookupParser!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < R , Parser > From < R > for DebugLookup < R , Parser > where R : Reader , Parser : LookupParser < R > , { fn from (input_buffer : R) -> Self { DebugLookup { input_buffer , phantom : PhantomData , } } }
    };
}

impl_472!();