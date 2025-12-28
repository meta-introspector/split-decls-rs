macro_rules! macro_363 {
    () => {
        cfg_if ! { if # [cfg (feature = "simd-accel")] { use :: core :: intrinsics :: likely ; use :: core :: intrinsics :: unlikely ; } else { # [inline (always)] fn likely (b : bool) -> bool { b } # [inline (always)] fn unlikely (b : bool) -> bool { b } } }
    };
}

macro_363!()