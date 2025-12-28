macro_rules! deps {
    () => {
        WallTime!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl WallTime { const NAME : & 'static str = "wall-time" ; pub fn new () -> Self { WallTime { start : Instant :: now () , } } # [inline] fn since_start (& self) -> u64 { self . start . elapsed () . as_nanos () as u64 } }
    };
}

impl_5!();