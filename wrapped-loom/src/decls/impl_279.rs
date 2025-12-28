macro_rules! deps {
    () => {
        Barrier!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl Barrier { # [doc = " `std::sync::Barrier` is not supported yet in Loom. This stub is provided just"] # [doc = " to make the code to compile."] pub fn new (_n : usize) -> Self { unimplemented ! ("std::sync::Barrier is not supported yet in Loom.") } # [doc = " `std::sync::Barrier` is not supported yet in Loom. This stub is provided just"] # [doc = " to make the code to compile."] pub fn wait (& self) -> std :: sync :: BarrierWaitResult { unimplemented ! ("std::sync::Barrier is not supported yet in Loom.") } }
    };
}

impl_279!()