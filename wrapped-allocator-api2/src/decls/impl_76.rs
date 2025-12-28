macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] # [cfg (all (feature = "fresh-rust" , not (feature = "std")))] impl Clone for Box < core :: ffi :: CStr > { # [inline] fn clone (& self) -> Self { (* * self) . into () } }
    };
}

impl_76!()