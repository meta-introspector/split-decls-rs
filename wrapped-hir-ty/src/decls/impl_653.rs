macro_rules! deps {
    () => {
        RustcFieldIdx!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        impl rustc_index :: Idx for RustcFieldIdx { fn new (idx : usize) -> Self { RustcFieldIdx (Idx :: from_raw (RawIdx :: from (idx as u32))) } fn index (self) -> usize { u32 :: from (self . 0 . into_raw ()) as usize } }
    };
}

impl_653!();