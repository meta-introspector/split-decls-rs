macro_rules! deps {
    () => {
        RustcFieldIdx!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        impl RustcFieldIdx { pub fn new (idx : usize) -> Self { RustcFieldIdx (Idx :: from_raw (RawIdx :: from (idx as u32))) } }
    };
}

impl_652!();