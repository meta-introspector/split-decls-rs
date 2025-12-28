macro_rules! deps {
    () => {
        SimpleCx!();
        FullCx!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'll , 'tcx > Deref for FullCx < 'll , 'tcx > { type Target = SimpleCx < 'll > ; # [inline] fn deref (& self) -> & Self :: Target { & self . scx } }
    };
}

impl_209!()