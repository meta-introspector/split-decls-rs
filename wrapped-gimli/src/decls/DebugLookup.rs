macro_rules! deps {
    () => {
        Reader!();
        LookupParser!();
    };
}

macro_rules! DebugLookup {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct DebugLookup < R , Parser > where R : Reader , Parser : LookupParser < R > , { input_buffer : R , phantom : PhantomData < Parser > , }
    };
}

DebugLookup!()