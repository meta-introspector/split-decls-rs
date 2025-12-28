macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        # [cfg (not (no_global_oom_handling))] # [cfg (feature = "std")] impl Clone for Box < std :: ffi :: CStr > { # [inline] fn clone (& self) -> Self { (* * self) . into () } }
    };
}

impl_74!()